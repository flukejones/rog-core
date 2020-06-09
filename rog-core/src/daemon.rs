use crate::{
    animatrix_control::{AniMeWriter, AnimatrixCommand},
    config::Config,
    laptops::match_laptop,
    led_control::{AuraCommand, LedWriter},
    rog_dbus::dbus_create_tree,
    rogcore::*,
};

use dbus::{channel::Sender, nonblock::Process};

use dbus_tokio::connection;
use log::{error, info, warn};
use rog_client::{DBUS_IFACE, DBUS_PATH};
use std::error::Error;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub(super) type FanModeType = Arc<Mutex<Option<u8>>>;
pub(super) type LedMsgType = Arc<Mutex<Option<Vec<u8>>>>;
pub(super) type NestedVecType = Arc<Mutex<Option<Vec<Vec<u8>>>>>;

// Timing is such that:
// - interrupt write is minimum 1ms (sometimes lower)
// - read interrupt must timeout, minimum of 1ms
// - for a single usb packet, 2ms total.
// - to maintain constant times of 1ms, per-key colours should use
//   the effect endpoint so that the complete colour block is written
//   as fast as 1ms per row of the matrix inside it. (10ms total time)
//
// DBUS processing takes 6ms if not tokiod
pub async fn start_daemon() -> Result<(), Box<dyn Error>> {
    let laptop = match_laptop();
    let mut config = Config::default().load();
    info!("Config loaded");

    let mut rogcore = RogCore::new(
        laptop.usb_vendor(),
        laptop.usb_product(),
        laptop.led_endpoint(),
    )
    .map_or_else(
        |err| {
            error!("{}", err);
            panic!("{}", err);
        },
        |daemon| {
            info!("RogCore loaded");
            daemon
        },
    );

    // Reload settings
    rogcore
        .fan_mode_reload(&mut config)
        .await
        .unwrap_or_else(|err| warn!("Fan mode: {}", err));
    let mut led_writer = LedWriter::new(
        rogcore.get_raw_device_handle(),
        laptop.led_endpoint(),
        (laptop.min_led_bright(), laptop.max_led_bright()),
        laptop.supported_modes().to_owned(),
    );
    led_writer
        .do_command(AuraCommand::ReloadLast, &mut config)
        .await?;

    // Possible Animatrix
    let mut animatrix_writer = None;
    if laptop.support_animatrix() {
        if let Ok(dev) = AniMeWriter::new() {
            animatrix_writer = Some(dev);
            info!("Device has an AniMe Matrix display");
        }
    }

    // Set up the mutexes
    let config = Arc::new(Mutex::new(config));
    let (resource, connection) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    connection
        .request_name(DBUS_IFACE, false, true, true)
        .await?;

    let (aura_command_send, aura_command_recv) = mpsc::sync_channel::<AuraCommand>(1);

    let (tree, input, effect, animatrix_img, fan_mode, effect_cancel_signal) = dbus_create_tree();
    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&*connection);

    // Keyboard reader goes in separate task because we want a high interrupt timeout
    // and don't want that to hold up other tasks, or miss keystrokes
    let keyboard_reader = KeyboardReader::new(
        rogcore.get_raw_device_handle(),
        laptop.key_endpoint(),
        laptop.key_filter().to_owned(),
    );

    let config1 = config.clone();
    // start the keyboard reader and laptop-action loop
    let key_read_handle = tokio::spawn(async move {
        loop {
            // Fan mode
            if let Ok(mut lock) = fan_mode.try_lock() {
                if let Some(n) = lock.take() {
                    let mut config = config1.lock().await;
                    rogcore
                        .fan_mode_set(n, &mut config)
                        .unwrap_or_else(|err| warn!("{:?}", err));
                }
            }
            let acs = aura_command_send.clone();
            let data = keyboard_reader.poll_keyboard().await;
            if let Some(bytes) = data {
                laptop
                    .run(&mut rogcore, &config1, bytes, acs)
                    .await
                    .unwrap_or_else(|err| warn!("{:?}", err));
            }
        }
    });

    // start the LED writer loop
    let led_write_handle = tokio::spawn(async move {
        let mut time_mark = Instant::now();
        loop {
            connection.process_all();

            // Check if a key press issued a command
            let res = aura_command_recv.recv_timeout(Duration::from_micros(50));
            if let Ok(command) = res {
                let mut config = config.lock().await;
                led_writer
                    .do_command(command, &mut config)
                    .await
                    .unwrap_or_else(|err| warn!("{:?}", err));

                connection
                    .send(
                        effect_cancel_signal
                            .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                            .append1(true),
                    )
                    .unwrap_or_else(|_| 0);
                // Clear any possible queued effect
                let mut effect = effect.lock().await;
                *effect = None;
                time_mark = Instant::now();
            } else {
                // Check if single mode
                if let Ok(mut lock) = input.try_lock() {
                    if let Some(bytes) = lock.take() {
                        if !bytes.is_empty() {
                            let mut config = config.lock().await;
                            led_writer
                                .do_command(AuraCommand::WriteBytes(bytes), &mut config)
                                .await
                                .unwrap_or_else(|err| warn!("{:?}", err));
                            // Also cancel any effect client
                            connection
                                .send(
                                    effect_cancel_signal.msg(&DBUS_PATH.into(), &DBUS_IFACE.into()),
                                )
                                .unwrap();
                            time_mark = Instant::now();
                        }
                    }
                }
                // Write a colour block
                let mut effect_lock = effect.lock().await;
                if let Some(effect) = effect_lock.take() {
                    if effect.len() == 11 {
                        let mut config = config.lock().await;
                        if effect.len() > 4 {
                            led_writer
                                .do_command(AuraCommand::WriteEffect(effect), &mut config)
                                .await
                                .unwrap_or_else(|err| warn!("{:?}", err));
                        } else {
                            led_writer
                                .do_command(AuraCommand::WriteMultizone(effect), &mut config)
                                .await
                                .unwrap_or_else(|err| warn!("{:?}", err));
                        }
                        time_mark = Instant::now();
                    }
                }
            }

            // If animatrix is supported, try doing a write
            if let Some(writer) = animatrix_writer.as_mut() {
                let mut image_lock = animatrix_img.lock().await;
                if let Some(image) = image_lock.take() {
                    writer
                        .do_command(AnimatrixCommand::WriteImage(image))
                        .await
                        .unwrap_or_else(|err| warn!("{:?}", err));
                }
            }

            let now = Instant::now();
            // Cool-down steps
            // This block is to prevent the loop spooling as fast as possible and saturating the CPU
            if now.duration_since(time_mark).as_millis() > 500 {
                tokio::time::delay_for(Duration::from_millis(200)).await;
            } else if now.duration_since(time_mark).as_millis() > 100 {
                tokio::time::delay_for(Duration::from_millis(50)).await;
            } else {
                tokio::time::delay_for(Duration::from_micros(300)).await;
            }
        }
    });

    led_write_handle.await?;
    key_read_handle.await?;
    Ok(())
}
