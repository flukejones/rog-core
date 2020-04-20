use crate::{DBUS_IFACE, DBUS_PATH};
use arrayvec::*;
use dbus::{
    blocking::Connection,
    tree::{Factory, MethodErr},
};
use log::{error, info, warn};
use rog_lib::core::*;
use std::error::Error;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};
use uhid_fs::{Bus, CreateParams, UHIDDevice};

pub struct Daemon {
    rogcore: RogCore,
}

impl Daemon {
    pub fn new() -> Self {
        Daemon {
            rogcore: RogCore::new().map_or_else(
                |err| {
                    error!("{}", err);
                    panic!("{}", err);
                },
                |daemon| {
                    info!("RogCore loaded");
                    daemon
                },
            ),
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

                                    match daemon.borrow_mut().rogcore.aura_set_and_save(&bytes[..])
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

        let mut rd_data = ArrayVec::new();
        CONSUMER.iter().for_each(|x| rd_data.try_push(*x).unwrap());

        let virt = Rc::new(RefCell::new(VirtKeys {
            device: UHIDDevice::try_new(CreateParams {
                name: ArrayString::from("Virtual ROG buttons").unwrap(),
                phys: ArrayString::from("").unwrap(),
                uniq: ArrayString::from("").unwrap(),
                bus: Bus::USB,
                vendor: 0x0b05,
                product: 0x1866,
                version: 0,
                country: 0,
                rd_data,
            })
            .unwrap(),
        }));

        let mut key_buf = [0u8; 32];
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
            match borrowed_daemon.rogcore.poll_keyboard(&mut key_buf) {
                Ok(read) => {
                    // Doing this because the Laptop trait takes RogCore, but RogCore contains laptop
                    // and this makes the borrow checker unhappy, but it's safe for this
                    let mut rogcore = unsafe { &mut (*daemon.as_ptr()).rogcore };
                    let laptop = borrowed_daemon.rogcore.laptop();

                    if let Some(_count) = read {
                        let virt = virt.clone();
                        laptop
                            .do_hotkey_action(
                                &mut rogcore,
                                key_buf[1],
                                Box::new(move |input| virt.borrow_mut().press(input)),
                            )
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

pub struct VirtKeys<T: std::io::Write + std::io::Read> {
    pub device: UHIDDevice<T>,
}

impl<T: std::io::Write + std::io::Read> VirtKeys<T> {
    pub fn press(&mut self, key: u8) {
        let mut bytes = [0u8; 8];
        bytes[0] = 0x02;
        bytes[1] = key;
        // email button
        // bytes[1] = 0x8a;
        // bytes[2] = 0x01;
        let mut datavec = arrayvec::ArrayVec::new();
        bytes.iter().for_each(|x| datavec.try_push(*x).unwrap());

        self.device.send_input(datavec.clone()).unwrap();
        datavec[1] = 0;
        self.device.send_input(datavec).unwrap();
    }
}

pub const CONSUMER: [u8; 25] = [
    0x05, 0x0C, // Usage Page (Consumer)
    0x09, 0x01, // Usage (Consumer Control)
    0xA1, 0x01, // Collection (Application)
    0x85, 0x02, //   Report ID (2)
    0x19, 0x00, //   Usage Minimum (Unassigned)
    0x2A, 0x3C, 0x02, //   Usage Maximum (AC Format)
    0x15, 0x00, //   Logical Minimum (0)
    0x26, 0x3C, 0x02, //   Logical Maximum (572)
    0x75, 0x10, //   Report Size (16)
    0x95, 0x02, //   Report Count (2)
    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0xC0,
];

// Usage 04 for microphone
// Needs another Usage (80) for system control
// B5 for toggle int/ext display
// b2 for external
pub enum ConsumerKeys {
    VolUp = 0xe9,     // USAGE (Volume up)
    VolDown = 0xea,   // USAGE (Volume down)
    VolMute = 0xe2,   // USAGE (Volume mute)
    TrackNext = 0xb6, // USAGE (Track next)
    PlayToggl = 0xcd, // USAGE (Play/Pause)
    TrackPrev = 0xb5, // USAGE (Track prev)
    TrackStop = 0xb7,
    Power = 0x30,
    Reset = 0x31,
    Sleep = 0x32,        // USAGE (Sleep)
    BacklightInc = 0x6f, // USAGE (Backlight Inc)
    BacklightDec = 0x70, // USAGE (Backlight Dec)
    BacklightTog = 0x72, // USAGE (Backlight toggle? display toggle?)
}
