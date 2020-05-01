use crate::{config::Config, core::*, laptops::match_laptop};
use dbus::{
    nonblock::Process,
    tree::{Factory, MTSync, Method, MethodErr, Tree},
};
use dbus_tokio::connection;
use log::{error, info, warn};
use rog_aura::{DBUS_IFACE, DBUS_PATH};
use std::error::Error;
use std::sync::Arc;
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
    let mut config = Config::default().read();

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
    rogcore.reload(&mut config).await?;

    // Set up the mutexes
    let led_writer = Arc::new(Mutex::new(LedWriter::new(
        rogcore.get_raw_device_handle(),
        laptop.led_endpoint(),
    )));
    let config = Arc::new(Mutex::new(config));
    let (resource, connection) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    connection
        .request_name(DBUS_IFACE, false, true, true)
        .await?;

    let (tree, input, effect) = dbus_create_tree();
    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&*connection);

    let supported = Vec::from(laptop.supported_modes());
    let led_endpoint = laptop.led_endpoint();

    // Keyboard reader goes in separate task because we want a high interrupt timeout
    // and don't want that to hold up other tasks, or miss keystrokes
    let keyboard_reader = KeyboardReader::new(
        rogcore.get_raw_device_handle(),
        laptop.key_endpoint(),
        laptop.key_filter().to_owned(),
    );

    let led_writer1 = led_writer.clone();
    let config1 = config.clone();
    // start the keyboard reader and laptop-action loop
    let key_read_handle = tokio::spawn(async move {
        loop {
            let data = keyboard_reader.poll_keyboard().await;
            if let Some(bytes) = data {
                laptop
                    .run(&mut rogcore, &led_writer1, &config1, bytes)
                    .await
                    .map_err(|err| warn!("{:?}", err))
                    .unwrap();
            }
        }
    });

    // start the LED writer loop
    let led_write_handle = tokio::spawn(async move {
        let mut time_mark = Instant::now();
        loop {
            connection.process_all();

            let led_writer = led_writer.clone();
            if let Ok(mut lock) = input.try_lock() {
                if let Some(bytes) = lock.take() {
                    let mut led_writer = led_writer.lock().await;
                    let mut config = config.lock().await;
                    led_writer
                        .aura_set_and_save(&supported, &bytes, &mut config)
                        .await
                        .map_err(|err| warn!("{:?}", err))
                        .unwrap();
                    time_mark = Instant::now();
                }
            }

            // Write a colour block
            let led_writer = led_writer.clone();
            if let Ok(mut lock) = effect.try_lock() {
                // Spawn a writer
                if let Some(stuff) = lock.take() {
                    let led_writer = led_writer.lock().await;
                    led_writer
                        .async_write_effect(led_endpoint, stuff)
                        .await
                        .map_err(|err| warn!("{:?}", err))
                        .unwrap();
                    time_mark = Instant::now();
                }
            }

            let now = Instant::now();
            // Cool-down steps
            // This block is to prevent the loop spooling as fast as possible and saturating the CPU
            if now.duration_since(time_mark).as_millis() > 500 {
                std::thread::sleep(Duration::from_millis(200));
            } else if now.duration_since(time_mark).as_millis() > 100 {
                std::thread::sleep(Duration::from_millis(50));
            } else {
                std::thread::sleep(Duration::from_micros(300));
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
        .method("ledmessage", (), {
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

fn dbus_create_ledeffect_method(effect: EffectType) -> Method<MTSync, ()> {
    let factory = Factory::new_sync::<()>();
    factory
        // method for ledmessage
        .method("ledeffect", (), {
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
}

fn dbus_create_tree() -> (Tree<MTSync, ()>, LedMsgType, EffectType) {
    let input: LedMsgType = Arc::new(Mutex::new(None));
    let effect: EffectType = Arc::new(Mutex::new(None));

    let factory = Factory::new_sync::<()>();
    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).add(
            factory
                .interface(DBUS_IFACE, ())
                .add_m(dbus_create_ledmsg_method(input.clone()))
                .add_m(dbus_create_ledeffect_method(effect.clone())),
        ),
    );
    (tree, input, effect)
}
