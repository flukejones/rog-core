// Return show-stopping errors, otherwise map error to a log level

use crate::{aura::BuiltInModeByte, config::Config, error::AuraError, laptops::*};
use aho_corasick::AhoCorasick;
use gumdrop::Options;
use log::{debug, warn};
use rusb::DeviceHandle;
use std::cell::{Ref, RefCell};
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;
use sysfs_class::{Brightness, SysClass};

pub const LED_MSG_LEN: usize = 17;
static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &'static str = "]ASUS Tech.Inc."; // ] == 0x5d
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0, 0x08];
static LED_INIT4: &'static str = "^ASUS Tech.Inc."; // ^ == 0x5e
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0, 0x08];

// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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
    config: Config,
    laptop: RefCell<Box<dyn Laptop>>,
}

impl RogCore {
    pub fn new() -> Result<RogCore, AuraError> {
        let laptop = match_laptop()?;

        let mut dev_handle = RogCore::get_device(laptop.usb_vendor(), laptop.usb_product())?;
        dev_handle.set_active_configuration(0).unwrap_or(());

        let dev_config = dev_handle.device().config_descriptor(0).unwrap();
        // Interface with outputs
        let mut led_interface_num = 0;
        let mut keys_interface_num = 0;
        for iface in dev_config.interfaces() {
            for desc in iface.descriptors() {
                for endpoint in desc.endpoint_descriptors() {
                    if endpoint.address() == 0x83 {
                        keys_interface_num = desc.interface_number();
                    } else if endpoint.address() == laptop.led_iface_num() {
                        led_interface_num = desc.interface_number();
                        break;
                    }
                }
            }
        }

        dev_handle.set_auto_detach_kernel_driver(true).unwrap();

        Ok(RogCore {
            handle: dev_handle,
            initialised: false,
            led_interface_num,
            keys_interface_num,
            config: Config::default().read(),
            laptop: RefCell::new(laptop),
        })
    }

    pub fn laptop(&self) -> Ref<Box<dyn Laptop>> {
        self.laptop.borrow()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    fn get_device(
        vendor: u16,
        product: u16,
    ) -> Result<DeviceHandle<rusb::GlobalContext>, AuraError> {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
                return device.open().map_err(|err| AuraError::from(err));
            }
        }
        Err(AuraError::from(rusb::Error::NoDevice))
    }

    fn aura_write_messages(&mut self, messages: &[&[u8]]) -> Result<(), AuraError> {
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
            write(*message)?;
            write(&LED_SET)?;
        }
        // Changes won't persist unless apply is set
        write(&LED_APPLY)?;

        self.handle.release_interface(self.led_interface_num)?;
        Ok(())
    }

    pub fn aura_brightness_bytes(brightness: u8) -> Result<[u8; 17], AuraError> {
        // TODO: check brightness range
        let mut bright = [0u8; LED_MSG_LEN];
        bright[0] = 0x5a;
        bright[1] = 0xba;
        bright[2] = 0xc5;
        bright[3] = 0xc4;
        bright[4] = brightness;
        Ok(bright)
    }

    pub fn aura_set_and_save(&mut self, bytes: &[u8]) -> Result<(), AuraError> {
        let mode = BuiltInModeByte::from(bytes[3]);
        if self.laptop().supported_modes().contains(&mode) || bytes[1] == 0xba {
            let messages = [bytes];
            self.aura_write_messages(&messages)?;
            self.config.set_field_from(bytes);
            self.config.write();
            debug!("Wrote: {:X?}", bytes);
            return Ok(());
        }
        warn!("{:?} not supported", BuiltInModeByte::from(mode));
        Err(AuraError::NotSupported)
    }

    pub fn poll_keyboard(&mut self, buf: &mut [u8; 32]) -> Result<Option<usize>, AuraError> {
        self.handle.claim_interface(self.keys_interface_num)?;
        match self
            .handle
            .read_interrupt(0x83, buf, Duration::from_micros(10))
        {
            Ok(o) => {
                if buf[0] == self.laptop.borrow().hotkey_group_byte() {
                    self.handle.release_interface(self.keys_interface_num)?;
                    return Ok(Some(o));
                }
            }
            Err(err) => {
                self.handle.release_interface(self.keys_interface_num)?;
                return Err(AuraError::from(err));
            }
        }
        self.handle.release_interface(self.keys_interface_num)?;
        Ok(None)
    }

    pub fn suspend(&self) {
        std::process::Command::new("systemctl")
            .arg("suspend")
            .spawn()
            .map_or_else(|err| warn!("Failed to suspend: {}", err), |_| {});
    }

    pub fn toggle_airplane_mode(&self) {
        match Command::new("rfkill").arg("list").output() {
            Ok(output) => {
                if output.status.success() {
                    let patterns = &["yes"];
                    let ac = AhoCorasick::new(patterns);
                    if ac.earliest_find(output.stdout).is_some() {
                        Command::new("rfkill")
                            .arg("unblock")
                            .arg("all")
                            .spawn()
                            .map_or_else(
                                |err| warn!("Could not unblock rf devices: {}", err),
                                |_| {},
                            );
                    } else {
                        let _ = Command::new("rfkill")
                            .arg("block")
                            .arg("all")
                            .spawn()
                            .map_or_else(
                                |err| warn!("Could not block rf devices: {}", err),
                                |_| {},
                            );
                    }
                } else {
                    warn!("Could not list rf devices");
                }
            }
            Err(err) => {
                warn!("Could not list rf devices: {}", err);
            }
        }
    }
}

pub struct Backlight {
    backlight: sysfs_class::Backlight,
    step: u64,
    max: u64,
}

impl Backlight {
    pub fn new(id: &str) -> Result<Backlight, std::io::Error> {
        for bl in sysfs_class::Backlight::iter() {
            let bl = bl?;
            if bl.id() == id {
                let max = bl.max_brightness()?;
                let step = max / 50;
                return Ok(Backlight {
                    backlight: bl,
                    step,
                    max,
                });
            }
        }
        panic!("Backlight not found")
    }
    pub fn step_up(&self) {
        let brightness = self
            .backlight
            .brightness()
            .map_err(|err| warn!("Failed to fetch backlight level: {}", err))
            .unwrap();
        if brightness + self.step <= self.max {
            self.backlight
                .set_brightness(brightness + self.step)
                .map_or_else(
                    |err| warn!("Failed to increment backlight level: {}", err),
                    |_| {},
                );
        }
    }
    pub fn step_down(&self) {
        let brightness = self
            .backlight
            .brightness()
            .map_err(|err| warn!("Failed to fetch backlight level: {}", err))
            .unwrap();
        if brightness > self.step {
            self.backlight
                .set_brightness(brightness - self.step)
                .map_or_else(
                    |err| warn!("Failed to increment backlight level: {}", err),
                    |_| {},
                );
        }
    }
}

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
