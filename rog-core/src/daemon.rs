use crate::{
    config::Config,
    ctrl_anime::{AniMeWriter, AnimatrixCommand},
    ctrl_charge::CtrlCharge,
    ctrl_fan_cpu::CtrlFanAndCPU,
    ctrl_leds::LedWriter,
    dbus::dbus_create_tree,
    laptops::match_laptop,
};

use dbus::{channel::Sender, nonblock::Process, nonblock::SyncConnection, tree::Signal};

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
    let mut laptop = match_laptop();
    let mut config = Config::default().load(laptop.supported_modes());

    info!("Config loaded");

    let mut led_control = LedWriter::new(laptop.usb_product(), laptop.supported_modes().to_owned())
        .map_or_else(
            |err| {
                error!("{}", err);
                None
            },
            |ledwriter| {
                info!("LED Writer loaded");
                Some(ledwriter)
            },
        );

    let mut charge_control = CtrlCharge::new().map_or_else(
        |err| {
            error!("{}", err);
            None
        },
        |ledwriter| {
            info!("Charge control loaded");
            Some(ledwriter)
        },
    );

    let mut fan_control = CtrlFanAndCPU::new().map_or_else(
        |err| {
            error!("{}", err);
            None
        },
        |ledwriter| {
            info!("Fan & CPU control loaded");
            Some(ledwriter)
        },
    );

    // Reload settings
    if let Some(ctrlr) = fan_control.as_mut() {
        ctrlr
            .fan_mode_reload(&mut config)
            .unwrap_or_else(|err| warn!("Fan mode: {}", err));
    }

    if let Some(ctrlr) = charge_control.as_mut() {
        ctrlr
            .bat_charge_limit_reload(&mut config)
            .unwrap_or_else(|err| warn!("Battery charge limit: {}", err));
    }

    if let Some(writer) = led_control.as_mut() {
        writer
            .reload_last_builtin(&mut config)
            .await
            .unwrap_or_else(|err| warn!("Reload settings: {}", err));
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
        mut aura_command_recv,
        mut animatrix_recv,
        fan_mode,
        charge_limit,
        led_changed_signal,
        fanmode_signal,
        charge_limit_signal,
    ) = dbus_create_tree(config.clone());
    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&*connection);

    // Send boot signals
    send_boot_signals(
        connection.clone(),
        config.clone(),
        fanmode_signal.clone(),
        charge_limit_signal.clone(),
        led_changed_signal.clone(),
    )
    .await?;

    // For helping with processing signals
    start_signal_task(
        connection.clone(),
        config.clone(),
        fanmode_signal,
        charge_limit_signal,
    );

    // Keyboard reader goes in separate task because we want a high interrupt timeout
    // and don't want that to hold up other tasks, or miss keystrokes

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
            laptop.set_support_animatrix(false);
        }
    }

    // start the keyboard reader and laptop-action loop
    let config1 = config.clone();
    // spawning this in a function causes a segfault for reasons I haven't investigated yet
    tokio::spawn(async move {
        loop {
            // TODO: MAKE SYS COMMANDS OPERATE USING CHANNEL LIKE AURA MODES
            // Fan mode
            if let Some(ctrlr) = fan_control.as_mut() {
                let mut config = config1.lock().await;
                ctrlr
                    .fan_mode_check_change(&mut config)
                    .unwrap_or_else(|err| warn!("{:?}", err));

                if let Ok(mut lock) = fan_mode.try_lock() {
                    if let Some(n) = lock.take() {
                        let mut config = config1.lock().await;
                        ctrlr
                            .set_fan_mode(n, &mut config)
                            .unwrap_or_else(|err| warn!("{:?}", err));
                    }
                }
            }

            // Charge limit
            if let Some(ctrlr) = charge_control.as_mut() {
                if let Ok(mut lock) = charge_limit.try_lock() {
                    if let Some(n) = lock.take() {
                        let mut config = config1.lock().await;
                        ctrlr
                            .set_charge_limit(n, &mut config)
                            .unwrap_or_else(|err| warn!("{:?}", err));
                    }
                }
            }
            tokio::time::delay_for(std::time::Duration::from_millis(500)).await;
        }
    });

    // start the main loop
    loop {
        connection.process_all();

        while let Some(command) = aura_command_recv.recv().await {
            if let Some(writer) = led_control.as_mut() {
                let mut config = config.lock().await;
                match &command {
                    AuraModes::RGB(_) => {
                        writer
                            .do_command(command, &mut config)
                            .await
                            .unwrap_or_else(|err| warn!("{}", err));
                    }
                    _ => {
                        let json = serde_json::to_string(&command)?;
                        writer
                            .do_command(command, &mut config)
                            .await
                            .unwrap_or_else(|err| warn!("{}", err));
                        connection
                            .send(
                                led_changed_signal
                                    .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                                    .append1(json),
                            )
                            .unwrap_or_else(|_| 0);
                    }
                }
            }
        }
    }
}

fn start_signal_task(
    connection: Arc<SyncConnection>,
    config: Arc<Mutex<Config>>,
    fanmode_signal: Arc<Signal<()>>,
    charge_limit_signal: Arc<Signal<()>>,
) {
    tokio::spawn(async move {
        // Some small things we need to track, without passing all sorts of stuff around
        let mut last_fan_mode = config.lock().await.fan_mode;
        let mut last_charge_limit = config.lock().await.bat_charge_limit;
        loop {
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
    });
}

async fn send_boot_signals(
    connection: Arc<SyncConnection>,
    config: Arc<Mutex<Config>>,
    fanmode_signal: Arc<Signal<()>>,
    charge_limit_signal: Arc<Signal<()>>,
    led_changed_signal: Arc<Signal<()>>,
) -> Result<(), Box<dyn Error>> {
    let config = config.lock().await;
    if let Some(data) = config.get_led_mode_data(config.current_mode) {
        connection
            .send(
                led_changed_signal
                    .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                    .append1(serde_json::to_string(data)?),
            )
            .unwrap_or_else(|_| 0);
    }
    connection
        .send(
            fanmode_signal
                .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                .append1(config.fan_mode),
        )
        .unwrap_or_else(|_| 0);
    connection
        .send(
            charge_limit_signal
                .msg(&DBUS_PATH.into(), &DBUS_IFACE.into())
                .append1(config.bat_charge_limit),
        )
        .unwrap_or_else(|_| 0);
    Ok(())
}
