pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

use crate::{core::RogCore, laptops::match_laptop};
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use log::{error, info, warn};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
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
    let factory = Factory::new_fn::<()>();

    let input: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(None));
    let effect: Rc<RefCell<Option<Vec<Vec<u8>>>>> = Rc::new(RefCell::new(None));

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
                                if let Ok(mut lock) = input.try_borrow_mut() {
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
                                if let Ok(mut lock) = effect.try_borrow_mut() {
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
    tree.start_receive(&connection);

    //thread::spawn(move || loop {});

    let supported = Vec::from(laptop.supported_modes());
    loop {
        // A no-comp loop takes 2 milliseconds
        // With effect, up to 16ms
        // With single write, 3ms
        // Actual EC for keyboard seems to take longer to process, something like
        // 2ms per line in, so 20ms per colour block?
        //
        // rusb is slow. Wireshark caps show speed in nanoseconds
        //thread::sleep(Duration::from_millis(2));
        connection
            .process(Duration::from_millis(20))
            .unwrap_or_else(|err| {
                error!("{:?}", err);
                false
            });

        // 700u per write
        if let Ok(mut lock) = input.try_borrow_mut() {
            if let Some(bytes) = &*lock {
                // It takes up to 10 milliseconds to write a complete colour block here
                // let now = std::time::Instant::now();
                rogcore.aura_set_and_save(&supported, &bytes)?;
                *lock = None;
                // let after = std::time::Instant::now();
                // let diff = after.duration_since(now);
                // dbg!(diff.as_millis());
            }
        }

        if let Ok(mut lock) = effect.try_borrow_mut() {
            if let Some(bytes) = &*lock {
                // It takes up to 10 milliseconds to write a complete colour block and here
                // let now = std::time::Instant::now();
                for row in bytes {
                    rogcore.aura_write(&row)?;
                }
                *lock = None;
                // let after = std::time::Instant::now();
                // let diff = after.duration_since(now);
                // dbg!(diff.as_millis());
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
