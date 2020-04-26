pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

use crate::{core::RogCore, laptops::match_laptop};
use dbus::{
    nonblock::Process,
    tree::{Factory, MethodErr},
};
use dbus_tokio::connection;

use log::{error, info};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub async fn start_daemon() -> Result<(), Box<dyn Error>> {
    let laptop = match_laptop();
    let mut rogcore = RogCore::new(&*laptop).map_or_else(
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
    info!("Reloaded last saved settings");

    let (resource, connection) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    connection
        .request_name(DBUS_IFACE, false, true, false)
        .await?;

    let factory = Factory::new_sync::<()>();

    let input: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let effect: Arc<Mutex<Option<Vec<Vec<u8>>>>> = Arc::new(Mutex::new(None));

    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).add(
            factory
                .interface(DBUS_IFACE, ())
                .add_m(
                    factory
                        // method for ledmessage
                        .method("ledmessage", (), {
                            let input = input.clone();

                            move |m| {
                                let bytes: Vec<u8> = m.msg.read1()?;
                                if let Ok(mut lock) = input.try_lock() {
                                    *lock = Some(bytes.to_vec());
                                    let mret = m
                                        .msg
                                        .method_return()
                                        .append1(&format!("Wrote {:x?}", bytes));
                                    return Ok(vec![mret]);
                                } else {
                                    return Err(MethodErr::failed(
                                        "Could not lock daemon for access",
                                    ));
                                }
                            }
                        })
                        .outarg::<&str, _>("reply")
                        .inarg::<Vec<u8>, _>("bytearray"),
                )
                .add_m(
                    factory
                        // method for ledmessage
                        .method("ledeffect", (), {
                            let effect = effect.clone();
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
                                    let mret =
                                        m.msg.method_return().append1(&format!("Got effect part"));
                                    return Ok(vec![mret]);
                                } else {
                                    return Err(MethodErr::failed(
                                        "Could not lock daemon for access",
                                    ));
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
                        .inarg::<Vec<u8>, _>("bytearray"),
                ),
        ),
    );

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&*connection);

    let supported = Vec::from(laptop.supported_modes());

    let key_buf: Arc<Mutex<Option<[u8; 32]>>> = Arc::new(Mutex::new(None));
    let handle = unsafe { &*(rogcore.get_raw_device_handle()) };
    let endpoint = laptop.key_endpoint();
    let report_filter_bytes = laptop.key_filter().to_owned();

    let key_buf1 = key_buf.clone();
    // This is *not* safe
    tokio::spawn(async move {
        loop {
            let data = RogCore::poll_keyboard(handle, endpoint, report_filter_bytes.clone()).await;
            if let Some(stuff) = data {
                if let Ok(mut lock) = key_buf1.lock() {
                    lock.replace(stuff);
                }
            }
        }
    });

    // When any action occurs this time is reset
    let mut time_mark = Instant::now();
    loop {
        // Timing is such that:
        // - interrupt write is minimum 1ms (sometimes lower)
        // - read interrupt must timeout, minimum of 1ms
        // - for a single usb packet, 2ms total.
        // - to maintain constant times of 1ms, per-key colours should use
        //   the effect endpoint so that the complete colour block is written
        //   as fast as 1ms per row of the matrix inside it. (10ms total time)

        // DBUS processing takes 6ms....
        connection.process_all();

        // 700u per write
        if let Ok(mut lock) = input.try_lock() {
            if let Some(bytes) = &*lock {
                // It takes up to 20 milliseconds to write a complete colour block here
                rogcore.aura_set_and_save(&supported, &bytes)?;
                *lock = None;
                time_mark = Instant::now();
            }
        }

        if let Ok(mut lock) = effect.lock() {
            if lock.is_some() {
                let effect = lock.take();
                rogcore.aura_write_effect(effect.unwrap())?;
                time_mark = Instant::now();
            }
        }

        if let Ok(mut lock) = key_buf.try_lock() {
            if let Some(bytes) = *lock {
                // this takes at least 10ms per colour block
                match laptop.run(&mut rogcore, bytes) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("{:?}", err);
                        panic!("Force crash for systemd to restart service")
                    }
                }
                *lock = None;
                time_mark = Instant::now();
            }
        }

        let now = Instant::now();
        // Cool-down steps
        if now.duration_since(time_mark).as_millis() > 500 {
            std::thread::sleep(Duration::from_millis(100));
        } else if now.duration_since(time_mark).as_millis() > 20 {
            std::thread::sleep(Duration::from_millis(20));
        } else {
            std::thread::sleep(Duration::from_micros(400));
        }
    }
}
