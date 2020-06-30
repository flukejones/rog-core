use crate::{
    animatrix_control::{AniMeWriter, AnimatrixCommand},
    config::Config,
    laptops::match_laptop,
    led_control::LedWriter,
    rog_dbus::dbus_create_tree,
    rogcore::*,
};

use dbus::{channel::Sender, nonblock::Process};

use dbus_tokio::connection;
use log::{error, info, warn};
use rog_client::{aura_modes::AuraModes, DBUS_IFACE, DBUS_NAME, DBUS_PATH};
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
    let mut config = Config::default().load(laptop.supported_modes());

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
        laptop.supported_modes().to_owned(),
    );
    led_writer
        .reload_last_builtin(&config)
        .await
        .unwrap_or_else(|err| warn!("Reload settings: {}", err));

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

    // Possible Animatrix
    if laptop.support_animatrix() {
        if let Ok(mut animatrix_writer) = AniMeWriter::new() {
            info!("Device has an AniMe Matrix display");
            tokio::spawn(async move {
                while let Some(image) = animatrix_recv.recv().await {
                    animatrix_writer
                        .do_command(AnimatrixCommand::WriteImage(image))
                        .await
                        .unwrap_or_else(|err| warn!("{}", err));
                }
            });
        }
    }

    let config1 = config.clone();
    // start the keyboard reader and laptop-action loop
    tokio::spawn(async move {
        loop {
            // Fan mode
            if let Ok(mut lock) = fan_mode.try_lock() {
                if let Some(n) = lock.take() {
                    let mut config = config1.lock().await;
                    rogcore
                        .set_fan_mode(n, &mut config)
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
                    .unwrap_or_else(|err| warn!("{}", err));
            }
        }
    });

    // For helping with processing signals
    let connection1 = connection.clone();
    let config1 = config.clone();
    tokio::spawn(async move {
        // Some small things we need to track, without passing all sorts of stuff around
        let mut last_fan_mode = config1.lock().await.fan_mode;
        let mut last_charge_limit = config1.lock().await.bat_charge_limit;
        loop {
            // Use tokio sleep to not hold up other threads
            tokio::time::delay_for(std::time::Duration::from_millis(500)).await;

            let config = config1.lock().await;
            if config.fan_mode != last_fan_mode {
                last_fan_mode = config.fan_mode;
                connection1
                    .send(
                        fanmode_signal
                            .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                            .append1(last_fan_mode),
                    )
                    .unwrap_or_else(|_| 0);
            }
            if config.bat_charge_limit != last_charge_limit {
                last_charge_limit = config.bat_charge_limit;
                connection1
                    .send(
                        charge_limit_signal
                            .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                            .append1(last_charge_limit),
                    )
                    .unwrap_or_else(|_| 0);
            }
        }
    });

    // start the main loop
    loop {
        connection.process_all();

        while let Some(command) = aura_command_recv.recv().await {
            let mut config = config.lock().await;
            match &command {
                AuraModes::RGB(_) => {
                    led_writer
                        .do_command(command, &mut config)
                        .await
                        .unwrap_or_else(|err| warn!("{}", err));
                }
                _ => {
                    let json = serde_json::to_string(&command)?;
                    led_writer
                        .do_command(command, &mut config)
                        .await
                        .unwrap_or_else(|err| warn!("{}", err));
                    connection
                        .send(
                            effect_cancel_signal
                                .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                                .append1(json),
                        )
                        .unwrap_or_else(|_| 0);
                }
            }
        }
    }
}
