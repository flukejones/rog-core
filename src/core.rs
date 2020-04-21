// Return show-stopping errors, otherwise map error to a log level

use crate::{
    aura::BuiltInModeByte, config::Config, error::AuraError, laptops::*, virt_device::VirtKeys,
};
use aho_corasick::AhoCorasick;
use gumdrop::Options;
use log::{error, warn};
use rusb::DeviceHandle;
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
pub(crate) struct RogCore {
    handle: DeviceHandle<rusb::GlobalContext>,
    initialised: bool,
    led_endpoint: u8,
    keys_endpoint: u8,
    config: Config,
    virt_keys: VirtKeys,
}

impl RogCore {
    pub(crate) fn new(laptop: &dyn Laptop) -> Result<RogCore, AuraError> {
        let mut dev_handle = RogCore::get_device(laptop.usb_vendor(), laptop.usb_product())?;
        dev_handle.set_active_configuration(0).unwrap_or(());

        let dev_config = dev_handle.device().config_descriptor(0).unwrap();
        // Interface with outputs
        let mut led_interface_num = 0;
        let mut keys_interface_num = 0;
        for iface in dev_config.interfaces() {
            for desc in iface.descriptors() {
                for endpoint in desc.endpoint_descriptors() {
                    if endpoint.address() == laptop.key_endpoint() {
                        keys_interface_num = desc.interface_number();
                    } else if endpoint.address() == laptop.led_endpoint() {
                        led_interface_num = desc.interface_number();
                        break;
                    }
                }
            }
        }

        dev_handle.set_auto_detach_kernel_driver(true).unwrap();
        dev_handle
            .claim_interface(keys_interface_num)
            .map_err(|err| AuraError::UsbError(err))?;

        Ok(RogCore {
            handle: dev_handle,
            initialised: false,
            led_endpoint: led_interface_num,
            keys_endpoint: laptop.key_endpoint(),
            config: Config::default().read(),
            virt_keys: VirtKeys::new(),
        })
    }

    pub(crate) fn virt_keys(&mut self) -> &mut VirtKeys {
        &mut self.virt_keys
    }

    pub(crate) fn config(&self) -> &Config {
        &self.config
    }

    pub(crate) fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    fn get_device(
        vendor: u16,
        product: u16,
    ) -> Result<DeviceHandle<rusb::GlobalContext>, AuraError> {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
                return device.open().map_err(|err| AuraError::UsbError(err));
            }
        }
        Err(AuraError::UsbError(rusb::Error::NoDevice))
    }

    fn aura_write(&mut self, message: &[u8]) -> Result<(), AuraError> {
        self.handle
            .write_control(0x21, 0x09, 0x035D, 0, message, Duration::new(0, 5))
            .map_err(|err| AuraError::UsbError(err))?;
        Ok(())
    }

    fn aura_write_messages(&mut self, messages: &[&[u8]]) -> Result<(), AuraError> {
        self.handle
            .claim_interface(self.led_endpoint)
            .map_err(|err| AuraError::UsbError(err))?;

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

        self.handle
            .release_interface(self.led_endpoint)
            .map_err(|err| AuraError::UsbError(err))?;
        Ok(())
    }

    /// Write the bytes read from the device interrupt to the buffer arg, and returns the
    /// count of bytes written
    ///
    /// `report_filter_bytes` is used to filter the data read from the interupt so
    /// only the relevant byte array is returned.
    pub(crate) fn poll_keyboard(&mut self, report_filter_bytes: &[u8]) -> Option<[u8; 32]> {
        let mut buf = [0u8; 32];
        match self
            .handle
            .read_interrupt(self.keys_endpoint, &mut buf, Duration::from_micros(1))
        {
            Ok(_) => {
                if report_filter_bytes.contains(&buf[0]) {
                    return Some(buf);
                }
            }
            Err(err) => {
                error!("Failed to read keyboard interrupt: {:?}", err);
            }
        }
        None
    }

    /// A direct call to systemd to suspend the PC.
    ///
    /// This avoids desktop environments being required to handle it
    /// (which means it works while in a TTY also)
    pub(crate) fn suspend_with_systemd(&self) {
        std::process::Command::new("systemctl")
            .arg("suspend")
            .spawn()
            .map_or_else(|err| warn!("Failed to suspend: {}", err), |_| {});
    }

    /// A direct call to rfkill to suspend wireless devices.
    ///
    /// This avoids desktop environments being required to handle it (which
    /// means it works while in a TTY also)
    pub(crate) fn toggle_airplane_mode(&self) {
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

    pub fn aura_set_and_save(
        &mut self,
        supported_modes: &[BuiltInModeByte],
        bytes: &[u8],
    ) -> Result<(), AuraError> {
        let mode = BuiltInModeByte::from(bytes[3]);
        if supported_modes.contains(&mode) || bytes[1] == 0xba {
            let messages = [bytes];
            self.aura_write_messages(&messages)?;
            self.config.set_field_from(bytes);
            self.config.write();
            return Ok(());
        }
        warn!("{:?} not supported", BuiltInModeByte::from(mode));
        Err(AuraError::NotSupported)
    }
}

pub(crate) struct Backlight {
    backlight: sysfs_class::Backlight,
    step: u64,
    max: u64,
}

impl Backlight {
    pub(crate) fn new(id: &str) -> Result<Backlight, std::io::Error> {
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
    pub(crate) fn step_up(&self) {
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
    pub(crate) fn step_down(&self) {
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

pub fn aura_brightness_bytes(brightness: u8) -> [u8; 17] {
    // TODO: check brightness range
    [
        0x5A, 0xBA, 0xC5, 0xC4, brightness, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}
