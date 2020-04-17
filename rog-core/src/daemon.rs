use crate::{DBUS_IFACE, DBUS_PATH};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use rog_lib::core::RogCore;
use rog_lib::hotkeys::*;
use std::error::Error;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

pub struct Daemon {
    rogcore: RogCore,
    hotkeys: Box<dyn Laptop>,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            rogcore: RogCore::new().expect("Could not start RogCore"),
            hotkeys: Box::new(LaptopGX502GW::new()),
        }
    }

    pub fn start() -> Result<(), Box<dyn Error>> {
        let mut connection = Connection::new_system().expect("Could not set up dbus system");
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
                                    let s = format!("Wrote {:x?}", bytes);

                                    let supported =
                                        unsafe { (*daemon.as_ptr()).hotkeys.supported_modes() };
                                    let mut daemon = daemon.borrow_mut();
                                    match daemon.rogcore.aura_set_and_save(&bytes[..], supported) {
                                        Ok(_) => {
                                            let mret = m.msg.method_return().append1(s);
                                            Ok(vec![mret])
                                        }
                                        Err(err) => Err(MethodErr::failed(&err)),
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
        loop {
            connection.process(Duration::from_millis(1))?;
            // READ KEYBOARD
            // TODO: this needs to move to a thread, but there is unsafety
            match daemon.borrow_mut().rogcore.poll_keyboard(&mut key_buf) {
                Ok(read) => {
                    let hotkeys = unsafe { &mut (*daemon.as_ptr()).hotkeys };
                    let mut rogcore = unsafe { &mut (*daemon.as_ptr()).rogcore };

                    if let Some(_count) = read {
                        if key_buf[0] == hotkeys.hotkey_group_byte() {
                            hotkeys.do_hotkey_action(&mut rogcore, key_buf[1]);
                        }
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}
