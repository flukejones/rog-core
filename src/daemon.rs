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
use std::thread;
use std::time::Duration;

pub fn start_daemon() -> Result<(), Box<dyn Error>> {
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
    rogcore.reload()?;

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

    let input: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).add(
            factory.interface(DBUS_IFACE, ()).add_m(
                factory
                    // method for ledmessage
                    .method("ledmessage", (), {
                        let input = input.clone();

                        move |m| {
                            let bytes: Vec<u8> = m.msg.read1()?;
                            if let Ok(mut lock) = input.lock() {
                                *lock = Some(bytes.to_vec());
                                let mret = m
                                    .msg
                                    .method_return()
                                    .append1(&format!("Wrote {:x?}", bytes));
                                return Ok(vec![mret]);
                            } else {
                                return Err(MethodErr::failed("Could not lock daemon for access"));
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

    //thread::spawn(move || loop {});

    let supported = Vec::from(laptop.supported_modes());
    loop {
        //thread::sleep(Duration::from_millis(2));
        connection
            .process(Duration::from_millis(20))
            .unwrap_or_else(|err| {
                error!("{:?}", err);
                false
            });

        if let Ok(mut lock) = input.try_lock() {
            if let Some(bytes) = &*lock {
                rogcore.aura_set_and_save(&supported, &bytes)?;
                *lock = None;
            }
        }

        match laptop.run(&mut rogcore) {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                panic!("Force crash for systemd to restart service")
            }
        }
    }
}
