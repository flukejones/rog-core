use crate::error::AuraError;
use gumdrop::Options;
use rusb::{DeviceHandle, Error};
use std::str::FromStr;
use std::time::Duration;

pub const LED_MSG_LEN: usize = 17;
static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &'static str = "]ASUS Tech.Inc."; // ] == 0x5d
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0, 0x08];
static LED_INIT4: &'static str = "^ASUS Tech.Inc."; // ^ == 0x5e
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0, 0x08];

// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Debug, Options)]
pub struct LedBrightness {
    level: u8,
}
impl LedBrightness {
    pub fn level(&self) -> u8 {
        self.level
    }
}
impl FromStr for LedBrightness {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "off" => Ok(LedBrightness { level: 0x00 }),
            "low" => Ok(LedBrightness { level: 0x01 }),
            "med" => Ok(LedBrightness { level: 0x02 }),
            "high" => Ok(LedBrightness { level: 0x03 }),
            _ => {
                println!("Missing required argument, must be one of:\noff,low,med,high\n");
                Err(AuraError::ParseBrightness)
            }
        }
    }
}

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

    fn aura_write_messages(&mut self, messages: &[&[u8]]) -> Result<(), Error> {
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
            write(*message)?;
            write(&LED_SET)?;
        }
        // Changes won't persist unless apply is set
        write(&LED_APPLY)?;

        self.handle.release_interface(self.led_interface_num)?;
        Ok(())
    }

    pub fn aura_brightness_bytes(brightness: u8) -> Result<[u8; 17], Error> {
        // TODO: check brightness range
        let mut bright = [0u8; LED_MSG_LEN];
        bright[0] = 0x5a;
        bright[1] = 0xba;
        bright[2] = 0xc5;
        bright[3] = 0xc4;
        bright[4] = brightness;
        Ok(bright)
    }

    pub fn aura_set_mode(&mut self, mode: &[u8]) -> Result<(), Error> {
        let messages = [mode];
        self.aura_write_messages(&messages)
    }

    pub fn poll_keyboard(&mut self) -> Result<[u8; 32], Error> {
        self.handle.claim_interface(self.keys_interface_num)?;

        let mut buf = [0; 32];
        match self
            .handle
            .read_interrupt(0x83, &mut buf, Duration::from_micros(10))
        {
            Ok(_) => {
                self.handle.release_interface(self.keys_interface_num)?;
                Ok(buf)
            }
            Err(err) => match err {
                //Error::Timeout => {}
                _ => return Err(err),
            },
        }
    }
}
