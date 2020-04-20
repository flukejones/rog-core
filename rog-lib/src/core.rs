// Return show-stopping errors, otherwise map error to a log level

use crate::{aura::BuiltInModeByte, config::Config, error::AuraError, laptops::*};
use aho_corasick::AhoCorasick;
use gumdrop::Options;
use hidapi::HidApi;
use log::{debug, error, info, warn};
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
    /// a handle to the HID device which controls LEDS
    dev_leds: Option<hidapi::HidDevice>,
    /// a handle to the HID device that the consumer + vendor specified
    /// keyboard buttons live on
    dev_cons: Option<hidapi::HidDevice>,
    initialised: bool,
    config: Config,
    laptop: RefCell<Box<dyn Laptop>>,
}

impl RogCore {
    pub fn new() -> Result<RogCore, AuraError> {
        let laptop = match_laptop()?;

        let mut dev_leds = None;
        let mut dev_cons = None;
        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    debug!("Checking: {:?}", device.path());
                    if device.vendor_id() == laptop.usb_vendor()
                        && device.product_id() == laptop.usb_product()
                    {
                        if device.path().to_string_lossy() == laptop.path_to_leds() {
                            dev_leds = Some(device.open_device(&api).unwrap());
                            info!("Found ROG LEDS");
                        }
                        if device.path().to_string_lossy() == laptop.path_to_cons() {
                            dev_cons = Some(device.open_device(&api).unwrap());
                            info!("Found ROG Consumer devices");
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }

        Ok(RogCore {
            dev_leds,
            dev_cons,
            initialised: false,
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

    // fn get_device(
    //     vendor: u16,
    //     product: u16,
    // ) -> Result<DeviceHandle<rusb::GlobalContext>, AuraError> {
    //     for device in rusb::devices().unwrap().iter() {
    //         let device_desc = device.device_descriptor().unwrap();
    //         if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
    //             return device.open().map_err(|err| AuraError::UsbError(err));
    //         }
    //     }
    //     Err(AuraError::UsbError(rusb::Error::NoDevice))
    // }

    fn aura_write(&mut self, message: &[u8]) -> Result<(), AuraError> {
        if let Some(aura) = &self.dev_leds {
            aura.write(message)
                .map_err(|err| AuraError::UsbError(err))?;
        }
        Ok(())
    }

    fn aura_write_messages(&mut self, messages: &[&[u8]]) -> Result<(), AuraError> {
        if !self.initialised {
            self.aura_write(&LED_INIT1)?;
            self.aura_write(LED_INIT2.as_bytes())?;
            self.aura_write(&LED_INIT3)?;
            self.aura_write(LED_INIT4.as_bytes())?;
            self.aura_write(&LED_INIT5)?;
            self.initialised = true;
        }

        for message in messages {
            self.aura_write(*message)?;
            self.aura_write(&LED_SET)?;
        }
        // Changes won't persist unless apply is set
        self.aura_write(&LED_APPLY)?;

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
        if let Some(rogcon) = &self.dev_cons {
            let res = rogcon
                .read_timeout(buf, 1)
                .map_err(|err| AuraError::UsbError(err))?;
            if self.laptop.borrow().hotkey_group_bytes().contains(&buf[0]) {
                return Ok(Some(res));
            }
        }
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
                let step = max / 500;
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
