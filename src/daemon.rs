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
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

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
    let factory = Factory::new_fnmut::<()>();

    let daemon = Rc::new(RefCell::new(rogcore));

    // We create a tree with one object path inside and make that path introspectable.
    let tree = factory.tree(()).add(
        factory.object_path(DBUS_PATH, ()).introspectable().add(
            // We add an interface to the object path...
            factory
                .interface(DBUS_IFACE, ())
                // ...and a method inside the interface
                .add_m(
                    factory
                        .method("ledmessage", (), {
                            let daemon = daemon.clone();
                            let supported = Vec::from(laptop.supported_modes());
                            move |m| {
                                // Reads the args passed to the method
                                let bytes: Vec<u8> = m.msg.read1()?;
                                match daemon
                                    .borrow_mut()
                                    .aura_set_and_save(&supported, &bytes[..])
                                {
                                    Ok(_) => {
                                        let s = format!("Wrote {:x?}", bytes);
                                        let mret = m.msg.method_return().append1(&s);
                                        Ok(vec![mret])
                                    }
                                    Err(err) => {
                                        warn!("{:?}", err);
                                        Err(MethodErr::failed(&err))
                                    }
                                }
                            }
                        })
                        // Input?
                        .outarg::<&str, _>("reply")
                        .inarg::<Vec<u8>, _>("bytearray"),
                ),
        ),
    );

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&connection);

    loop {
        connection
            .process(Duration::from_millis(10))
            .unwrap_or_else(|err| {
                error!("{:?}", err);
                false
            });

        // TODO: this needs to move to a thread, but there is unsafety
        let mut borrowed_daemon = daemon.borrow_mut();
        laptop.run(&mut borrowed_daemon).unwrap_or_else(|err| {
            error!("{:?}", err);
        });
    }
}
