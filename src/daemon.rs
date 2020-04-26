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
use std::time::Duration;

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
    println!("RELOADED");

    let (resource, connection) = connection::new_system_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    println!("CONN REQUEST");
    connection
        .request_name(DBUS_IFACE, false, true, false)
        .await?;
    println!("CONN REQUEST DONE");

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
                rogcore.aura_set_and_save(&supported, &bytes).await?;
                *lock = None;
            }
        }

        if let Ok(mut lock) = effect.try_lock() {
            if lock.is_some() {
                let effect = lock.take();
                rogcore.aura_write_effect(effect.unwrap()).await?;
            }
        }

        match laptop.run(&mut rogcore).await {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                panic!("Force crash for systemd to restart service")
            }
        }
        // When using an effect gen, 17-20ms is the min/max time to account for.
        // If DBUS didn't take so long we could get this down to 11-12ms.
    }
}
