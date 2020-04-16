use crate::{config::Config, core::RogCore, DBUS_IFACE, DBUS_PATH};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use std::error::Error;
use std::time::Duration;

pub fn start_daemon() -> Result<(), Box<dyn Error>> {
    let mut config = Config::default().read();
    let mut core = RogCore::new()?;
    core.aura_set_mode(&config.builtin)?;
    let bright = RogCore::aura_brightness_bytes(config.brightness)?;
    core.aura_set_mode(&bright)?;

    let mut c = Connection::new_system()?;
    c.request_name(DBUS_IFACE, false, true, false)?;

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fnmut::<()>();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(
        f.object_path(DBUS_PATH, ()).introspectable().add(
            // We add an interface to the object path...
            f.interface(DBUS_IFACE, ())
                // ...and a method inside the interface
                .add_m(
                    f.method("ledmessage", (), move |m| {
                        // Reads the args passed to the method
                        // Reads the args passed to the method
                        let bytes: Vec<u8> = m.msg.read1()?;
                        let s = format!("Shoving {:x?} in to keyboard!", bytes);

                        if let Ok(mut core) = RogCore::new() {
                            match core.aura_set_mode(&bytes[..]) {
                                Ok(_) => {
                                    // TODO: create statics out of header bytes
                                    if bytes[0] == 0x5a && bytes[1] == 0xba {
                                        config.brightness = bytes[4];
                                    } else if bytes[0] == 0x5d && bytes[1] == 0xb3 {
                                        config.builtin = bytes;
                                    }
                                    config.write();
                                    let mret = m.msg.method_return().append1(s);
                                    Ok(vec![mret])
                                }
                                Err(err) => Err(MethodErr::failed(&err)),
                            }
                        } else {
                            Err(MethodErr::failed("Failed to start RogCore"))
                        }
                    })
                    // Input?
                    .outarg::<&str, _>("reply")
                    .inarg::<Vec<u8>, _>("bytearray"),
                ),
        ),
    );

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&c);

    loop {
        c.process(Duration::from_secs(1))?;
    }
}
