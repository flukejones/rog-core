pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

use crate::{core::RogCore, laptops::match_laptop};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use log::{error, info, warn};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn start_daemon() -> Result<(), Box<dyn Error>> {
    let laptop = match_laptop();
    let rogcore = RogCore::new(&*laptop).map_or_else(
        |err| {
            error!("{}", err);
            panic!("{}", err);
        },
        |daemon| {
            info!("RogCore loaded");
            daemon
        },
    );

    let mut connection = Connection::new_system().map_or_else(
        |err| {
            error!("{:?}", err);
            panic!("{:?}", err);
        },
        |dbus| {
            info!("DBus connected");
            dbus
        },
    );
    connection.request_name(DBUS_IFACE, false, true, false)?;
    let factory = Factory::new_sync::<()>();

    let daemon = Arc::new(Mutex::new(rogcore));

    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).add(
            factory.interface(DBUS_IFACE, ()).add_m(
                factory
                    // method for ledmessage
                    .method("ledmessage", (), {
                        let daemon = daemon.clone();
                        let supported = Vec::from(laptop.supported_modes());
                        move |m| {
                            if let Ok(mut lock) = daemon.try_lock() {
                                let bytes: Vec<u8> = m.msg.read1()?;
                                match lock.aura_set_and_save(&supported, &bytes[..]) {
                                    Ok(_) => {
                                        let mret = m
                                            .msg
                                            .method_return()
                                            .append1(&format!("Wrote {:x?}", bytes));
                                        Ok(vec![mret])
                                    }
                                    Err(err) => {
                                        warn!("{:?}", err);
                                        Err(MethodErr::failed(&err))
                                    }
                                }
                            } else {
                                Err(MethodErr::failed("Could not lock daemon for access"))
                            }
                        }
                    })
                    .outarg::<&str, _>("reply")
                    .inarg::<Vec<u8>, _>("bytearray"),
            ),
        ),
    );

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive_send(&connection);

    loop {
        connection
            .process(Duration::from_millis(10))
            .unwrap_or_else(|err| {
                error!("{:?}", err);
                false
            });

        // TODO: this needs to move to a thread, but there is unsafety
        if let Ok(mut lock) = daemon.try_lock() {
            laptop.run(&mut lock).unwrap_or_else(|err| {
                error!("{:?}", err);
            });
        }
    }
}
