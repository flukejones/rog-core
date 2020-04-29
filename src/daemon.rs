pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

use crate::{
    core::RogCore,
    laptops::{match_laptop, Laptop},
};
use dbus::{
    nonblock::Process,
    tree::{Factory, MTSync, Method, MethodErr, Tree},
};
use dbus_tokio::connection;

use log::{error, info};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

type LedMsgType = Arc<tokio::sync::Mutex<Option<Vec<u8>>>>;
type EffectType = Arc<tokio::sync::Mutex<Option<Vec<Vec<u8>>>>>;

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
    rogcore.reload().await?;
    let usb_dev_handle = unsafe { &*(rogcore.get_raw_device_handle()) };
    let rogcore = Arc::new(tokio::sync::Mutex::new(Box::pin(rogcore)));

    let (resource, connection) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    connection
        .request_name(DBUS_IFACE, false, true, false)
        .await?;

    let (tree, input, effect) = dbus_create_tree();
    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&*connection);

    let supported = Vec::from(laptop.supported_modes());
    let led_endpoint = laptop.led_endpoint();

    {
        let keyboard_endpoint = laptop.key_endpoint();
        let report_filter_bytes = laptop.key_filter().to_owned();
        // This is *not* safe
        let rogcore = rogcore.clone();
        tokio::spawn(async move {
            loop {
                let data =
                    RogCore::poll_keyboard(usb_dev_handle, keyboard_endpoint, &report_filter_bytes)
                        .await;
                if let Some(bytes) = data {
                    let mut rogcore = rogcore.lock().await;
                    match laptop.run(&mut rogcore, bytes) {
                        Ok(_) => {}
                        Err(err) => {
                            error!("{:?}", err);
                            panic!("Force crash for systemd to restart service")
                        }
                    }
                }
            }
        });
    }

    let mut time_mark = Instant::now();
    loop {
        connection.process_all();

        if let Ok(mut lock) = input.try_lock() {
            if let Some(bytes) = lock.take() {
                let mut rogcore = rogcore.lock().await;
                rogcore.aura_set_and_save(&supported, &bytes)?;
                time_mark = Instant::now();
            }
        }

        // Write a colour block
        // Yank data out and drop lock quick (effect write takes 10ms)
        if let Ok(mut lock) = effect.try_lock() {
            // Spawn a writer
            if let Some(stuff) = lock.take() {
                tokio::spawn(async move {
                    RogCore::async_write_effect(usb_dev_handle, led_endpoint, stuff)
                        .await
                        .unwrap();
                });
                time_mark = Instant::now();
            }
        }

        let now = Instant::now();
        // Cool-down steps
        if now.duration_since(time_mark).as_millis() > 500 {
            std::thread::sleep(Duration::from_millis(200));
        } else if now.duration_since(time_mark).as_millis() > 100 {
            std::thread::sleep(Duration::from_millis(50));
        } else {
            std::thread::sleep(Duration::from_micros(5));
        }
    }
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
    let input: LedMsgType = Arc::new(tokio::sync::Mutex::new(None));
    let effect: EffectType = Arc::new(tokio::sync::Mutex::new(None));

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
