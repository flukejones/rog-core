use crate::aura::ModeMessage;
use rusb::{DeviceHandle, Error};
use std::time::Duration;

pub const LED_MSG_LEN: usize = 17;
pub static LED_SPEED_BYTES: [u8; 3] = [0xe1, 0xeb, 0xf5]; // maps to 1, 2, 3
static LED_BRIGHT_BYTES: [u8; 5] = [0x5a, 0xba, 0xc5, 0xc4, 0]; // Index 4 = set brightness
static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &'static str = "]ASUS Tech.Inc."; // ] == 0x5d
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0, 0x08];
static LED_INIT4: &'static str = "^ASUS Tech.Inc."; // ^ == 0x5e
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0, 0x08];

static LED_BUILTIN: [u8; 13] = [
    0x5d, 0xb3, 0x03, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00,
];

// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// bytes 0,1 set mode
// bytes 3,4,5 set to on,
// byte 6 increments (starts at 0x00, continues 0x10 to 0xA0) 0-10 << 4
// byte 7 resets (stays on 10 until byte6 == 0xA0 then flips to 08 in same packet as 0xA0)
// bytes 8-57 are bitfield for keys
static LED_SPECIAL: [u8; 64] = [
    0x5d, 0xbc, 0x00, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// ROG device controller
///
/// For the GX502GW the LED setup sequence looks like:
///
/// -` LED_INIT1`
/// - `LED_INIT3`
/// - `LED_INIT4`
/// - `LED_INIT2`
/// - `LED_INIT4`
pub struct RogCore {
    handle: DeviceHandle<rusb::GlobalContext>,
    initialised: bool,
    led_interface_num: u8,
    keys_interface_num: u8,
}

impl RogCore {
    pub fn new() -> Result<RogCore, Error> {
        let mut handle = RogCore::get_device(0x0B05, 0x1866)?;
        handle.set_active_configuration(0).unwrap_or(());

        let config = handle.device().config_descriptor(0).unwrap();
        // Interface with outputs
        let mut keys_interface_num = 0;
        let mut led_interface_num = 0;
        for iface in config.interfaces() {
            for desc in iface.descriptors() {
                for endpoint in desc.endpoint_descriptors() {
                    if endpoint.address() == 0x83 {
                        keys_interface_num = desc.interface_number();
                    } else if endpoint.address() == 0x81 {
                        led_interface_num = desc.interface_number();
                    }
                }
            }
        }

        handle.set_auto_detach_kernel_driver(true).unwrap();
        handle.set_auto_detach_kernel_driver(true).unwrap();

        // let iface_out = config
        //     .interfaces()
        //     .nth(keys_interface_num as usize)
        //     .unwrap();
        // let iface_led = config.interfaces().nth(led_interface_num as usize).unwrap();

        //handle.claim_interface(iface_out.number()).unwrap();
        //handle.claim_interface(iface_led.number()).unwrap();

        Ok(RogCore {
            handle,
            initialised: false,
            led_interface_num,
            keys_interface_num,
        })
    }

    fn get_device(vendor: u16, product: u16) -> Result<DeviceHandle<rusb::GlobalContext>, Error> {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
                return device.open();
            }
        }
        Err(Error::NoDevice)
    }

    fn aura_write_messages(&mut self, messages: &[[u8; LED_MSG_LEN]]) -> Result<(), Error> {
        self.handle.claim_interface(self.led_interface_num)?;
        // Declared as a zoomy so that it is hidden
        let write = |message: &[u8]| {
            self.handle
                .write_control(0x21, 0x09, 0x035D, 0, message, Duration::new(0, 5))
        };

        if !self.initialised {
            write(&LED_INIT1)?;
            write(LED_INIT2.as_bytes())?;
            write(&LED_INIT3)?;
            write(LED_INIT4.as_bytes())?;
            write(&LED_INIT5)?;
        }

        for message in messages {
            println!("{:x?}", &message);
            write(message)?;
            write(&LED_SET)?;
            //write(&LED_APPLY)?;
        }

        self.handle.release_interface(self.led_interface_num)?;
        Ok(())
    }

    pub fn aura_set_brightness(&mut self, brightness: u8) -> Result<(), Error> {
        let mut bright = [0u8; LED_MSG_LEN];
        bright[0] = 0x5a;
        bright[1] = 0xba;
        bright[2] = 0xc5;
        bright[3] = 0xc4;
        bright[4] = brightness;
        let messages = [bright];
        self.aura_write_messages(&messages)
    }

    pub fn aura_set_mode(&mut self, mode: ModeMessage) -> Result<(), Error> {
        let messages = [mode.0];
        self.aura_write_messages(&messages)
    }
}
