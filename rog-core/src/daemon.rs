use crate::{DBUS_IFACE, DBUS_PATH};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use log::{error, info, warn};
use rog_lib::{
    core::RogCore,
    laptops::{match_laptop, Laptop},
};
use std::error::Error;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

pub struct Daemon {
    rogcore: RogCore,
    laptop: Box<dyn Laptop>,
}

impl Daemon {
    pub fn new() -> Self {
        let laptop = match_laptop();

        Daemon {
            rogcore: RogCore::new(&*laptop).map_or_else(
                |err| {
                    error!("{}", err);
                    panic!("{}", err);
                },
                |daemon| {
                    info!("RogCore loaded");
                    daemon
                },
            ),
            laptop,
        }
    }

    pub fn start() -> Result<(), Box<dyn Error>> {
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

        let daemon = Self::new();
        let daemon = Rc::new(RefCell::new(daemon));

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
                                move |m| {
                                    // Reads the args passed to the method
                                    let bytes: Vec<u8> = m.msg.read1()?;
                                    let supported =
                                        Vec::from(daemon.borrow().laptop.supported_modes());
                                    match daemon
                                        .borrow_mut()
                                        .rogcore
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

        let mut key_buf = [0u8; 32];
        let hotkey_group_bytes = Vec::from(daemon.borrow().laptop.hotkey_group_bytes());
        loop {
            connection
                .process(Duration::from_millis(10))
                .unwrap_or_else(|err| {
                    error!("{:?}", err);
                    false
                });
            // READ KEYBOARD
            // TODO: this needs to move to a thread, but there is unsafety
            let mut borrowed_daemon = daemon.borrow_mut();
            match borrowed_daemon
                .rogcore
                .poll_keyboard(&hotkey_group_bytes, &mut key_buf)
            {
                Ok(read) => {
                    // Doing this because the Laptop trait takes RogCore, but RogCore contains laptop
                    // and this makes the borrow checker unhappy, but it's safe for this
                    let mut rogcore = unsafe { &mut (*daemon.as_ptr()).rogcore };

                    if let Some(_count) = read {
                        borrowed_daemon
                            .laptop
                            .do_hotkey_action(&mut rogcore, key_buf[1])
                            .unwrap_or_else(|err| {
                                warn!("{:?}", err);
                            });
                    }
                }
                Err(err) => error!("{:?}", err),
            }
        }
    }
}
