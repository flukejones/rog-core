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
use rog_client::{DBUS_IFACE, DBUS_NAME, DBUS_PATH};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(super) type DbusU8Type = Arc<Mutex<Option<u8>>>;

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
        laptop.key_endpoint(),
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
        .unwrap_or_else(|err| warn!("Fan mode: {}", err));
    rogcore
        .bat_charge_limit_reload(&mut config)
        .unwrap_or_else(|err| warn!("Battery charge limit: {}", err));

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
        .request_name(DBUS_NAME, false, true, true)
        .await?;

    let (
        tree,
        aura_command_sender,
        mut aura_command_recv,
        mut animatrix_recv,
        fan_mode,
        charge_limit,
        effect_cancel_signal,
        fanmode_signal,
        charge_limit_signal,
    ) = dbus_create_tree(config.clone());
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
    tokio::spawn(async move {
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
            // Charge limit
            if let Ok(mut lock) = charge_limit.try_lock() {
                if let Some(n) = lock.take() {
                    let mut config = config1.lock().await;
                    rogcore
                        .set_charge_limit(n, &mut config)
                        .unwrap_or_else(|err| warn!("{:?}", err));
                }
            }
            // Keyboard reads
            let data = keyboard_reader.poll_keyboard().await;
            if let Some(bytes) = data {
                laptop
                    .run(&mut rogcore, &config1, bytes, aura_command_sender.clone())
                    .await
                    .unwrap_or_else(|err| warn!("{:?}", err));
            }
        }
    });

    // If animatrix is supported, try doing a write
    tokio::spawn(async move {
        if let Some(writer) = animatrix_writer.as_mut() {
            while let Some(image) = animatrix_recv.recv().await {
                writer
                    .do_command(AnimatrixCommand::WriteImage(image))
                    .await
                    .unwrap_or_else(|err| warn!("{:?}", err));
            }
        }
    });

    let connection1 = connection.clone();
    let config1 = config.clone();
    tokio::spawn(async move {
        // Check if a key press issued a command
        while let Some(command) = aura_command_recv.recv().await {
            let mut config = config1.lock().await;
            match command {
                AuraCommand::WriteEffect(_) | AuraCommand::WriteMultizone(_) => led_writer
                    .do_command(command, &mut config)
                    .await
                    .unwrap_or_else(|err| warn!("{:?}", err)),
                _ => {
                    led_writer
                        .do_command(command, &mut config)
                        .await
                        .unwrap_or_else(|err| warn!("{:?}", err));
                    connection1
                        .send(effect_cancel_signal.msg(&DBUS_PATH.into(), &DBUS_IFACE.into()))
                        .unwrap_or_else(|_| 0);
                }
            }
        }
    });

    // Some small things we need to track, without passing all sorts of stuff around
    let mut last_fan_mode = config.lock().await.fan_mode;
    let mut last_charge_limit = config.lock().await.bat_charge_limit;
    // start the main loop
    loop {
        connection.process_all();
        // Use tokio sleep to not hold up other threads
        tokio::time::delay_for(std::time::Duration::from_millis(500)).await;

        let config = config.lock().await;
        if config.fan_mode != last_fan_mode {
            last_fan_mode = config.fan_mode;
            connection
                .send(
                    fanmode_signal
                        .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                        .append1(last_fan_mode),
                )
                .unwrap_or_else(|_| 0);
        }
        if config.bat_charge_limit != last_charge_limit {
            last_charge_limit = config.bat_charge_limit;
            connection
                .send(
                    charge_limit_signal
                        .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                        .append1(last_charge_limit),
                )
                .unwrap_or_else(|_| 0);
        }
    }
}
