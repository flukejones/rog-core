use crate::{
    config::Config,
    laptops::match_laptop,
    led_control::{AuraCommand, LedWriter},
    rogcore::*,
};

use dbus::{
    channel::Sender,
    nonblock::Process,
    tree::{Factory, MTSync, Method, MethodErr, Signal, Tree},
};

use dbus_tokio::connection;
use log::{error, info, warn};
use rog_aura::{DBUS_IFACE, DBUS_PATH};
use std::error::Error;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

type LedMsgType = Arc<Mutex<Option<Vec<u8>>>>;
type EffectType = Arc<Mutex<Option<Vec<Vec<u8>>>>>;

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

    let (tree, input, effect, effect_cancel_signal) = dbus_create_tree();
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

fn dbus_create_ledmsg_method(msg: LedMsgType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteBytes", (), {
            move |m| {
                let bytes: Vec<u8> = m.msg.read1()?;
                if let Ok(mut lock) = msg.try_lock() {
                    *lock = Some(bytes.to_vec());
                    let mret = m
                        .msg
                        .method_return()
                        .append1(&format!("Wrote {:x?}", bytes));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<Vec<u8>, _>("bytearray")
}

fn dbus_create_ledmultizone_method(effect: EffectType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteMultizone", (), {
            move |m| {
                if let Ok(mut lock) = effect.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte_array: Vec<Vec<u8>> =
                        vec![iter.read()?, iter.read()?, iter.read()?, iter.read()?];
                    *lock = Some(byte_array);
                    let mret = m.msg.method_return().append1(&format!("Got effect part"));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
}

fn dbus_create_ledeffect_method(effect: EffectType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("LedWriteEffect", (), {
            move |m| {
                if let Ok(mut lock) = effect.try_lock() {
                    let mut iter = m.msg.iter_init();
                    let byte_array: Vec<Vec<u8>> = vec![
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                        iter.read()?,
                    ];
                    *lock = Some(byte_array);
                    let mret = m.msg.method_return().append1(&format!("Got effect part"));
                    Ok(vec![mret])
                } else {
                    Err(MethodErr::failed("Could not lock daemon for access"))
                }
            }
        })
        .outarg::<&str, _>("reply")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
        .inarg::<Vec<u8>, _>("bytearray")
}

fn dbus_create_tree() -> (Tree<MTSync, ()>, LedMsgType, EffectType, Arc<Signal<()>>) {
    let input_bytes: LedMsgType = Arc::new(Mutex::new(None));
    let input_effect: EffectType = Arc::new(Mutex::new(None));

    let factory = Factory::new_sync::<()>();
    let effect_cancel_sig = Arc::new(factory.signal("LedCancelEffect", ()));
    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).add(
            factory
                .interface(DBUS_IFACE, ())
                .add_m(dbus_create_ledmsg_method(input_bytes.clone()))
                .add_m(dbus_create_ledmultizone_method(input_effect.clone()))
                .add_m(dbus_create_ledeffect_method(input_effect.clone()))
                .add_s(effect_cancel_sig.clone()),
        ),
    );
    (tree, input_bytes, input_effect, effect_cancel_sig)
}
