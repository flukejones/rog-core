use crate::{config::Config, rogcore::RogCore};
use rog_client::{
    aura_modes::{
        AuraModes, BREATHING, COMET, FLASH, HIGHLIGHT, LASER, MULTISTATIC, PULSE, RAIN, RAINBOW,
        RGB, RIPPLE, SINGLE, STAR, STROBE,
    },
    error::AuraError,
};
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use crate::virt_device::{ConsumerKeys, KeyboardKeys};
use log::{info, warn};

static HELP_ADDRESS: &str = "https://github.com/flukejones/rog-core";

/// The map of bytes which each fn+key combo emits (0x1866 device only)
pub enum FnKeys {
    Rog = 0x38,
    MicToggle = 0x7C,
    Fan = 0xAE,
    ScreenToggle = 0x35,
    ScreenBrightDn = 0x10,
    ScreenBrightUp = 0x20,
    TouchPadToggle = 0x6b,
    Sleep = 0x6C,
    AirplaneMode = 0x88,
    LedBrightUp = 0xC4,
    LedBrightDown = 0xC5,
    AuraPrevious = 0xB2,
    AuraNext = 0xB3,
    Calc = 0x92,
    None,
}

impl From<u8> for FnKeys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => FnKeys::Rog,
            0x7C => FnKeys::MicToggle,
            0xAE => FnKeys::Fan,
            0x35 => FnKeys::ScreenToggle,
            0x10 => FnKeys::ScreenBrightDn,
            0x20 => FnKeys::ScreenBrightUp,
            0x6b => FnKeys::TouchPadToggle,
            0x6C => FnKeys::Sleep,
            0x88 => FnKeys::AirplaneMode,
            0xC4 => FnKeys::LedBrightUp,
            0xC5 => FnKeys::LedBrightDown,
            0xB2 => FnKeys::AuraPrevious,
            0xB3 => FnKeys::AuraNext,
            0x90 => FnKeys::Calc,
            _ => FnKeys::None,
        }
    }
}

pub(crate) fn match_laptop() -> LaptopBase {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0b05 {
            match device_desc.product_id() {
                0x1869 | 0x1866 => return select_1866_device(device_desc.product_id()),
                0x1854 => {
                    info!("Found GL753 or similar");
                    return LaptopBase {
                        usb_vendor: 0x0B05,
                        usb_product: 0x1854,
                        report_filter_bytes: vec![0x5a, 0x02],
                        min_led_bright: 0x00,
                        max_led_bright: 0x03,
                        //from `lsusb -vd 0b05:1866`
                        led_endpoint: 0x04,
                        //from `lsusb -vd 0b05:1866`
                        key_endpoint: 0x83,
                        supported_modes: vec![SINGLE, BREATHING, STROBE],
                        support_animatrix: false,
                    };
                }
                _ => {}
            }
        }
    }
    panic!("could not match laptop");
}

fn select_1866_device(prod: u16) -> LaptopBase {
    let dmi = sysfs_class::DmiId::default();
    let board_name = dmi.board_name().expect("Could not get board_name");
    let prod_name = dmi.product_name().expect("Could not get board_name");

    info!("Product name: {}", prod_name.trim());
    info!("Board name: {}", board_name.trim());

    let mut laptop = LaptopBase {
        usb_vendor: 0x0B05,
        usb_product: prod,
        report_filter_bytes: vec![0x5a, 0x02],
        min_led_bright: 0x00,
        max_led_bright: 0x03,
        //from `lsusb -vd 0b05:1866`
        led_endpoint: 0x04,
        //from `lsusb -vd 0b05:1866`
        key_endpoint: 0x83,
        supported_modes: vec![],
        support_animatrix: false,
    };

    // GA401
    if board_name.starts_with("GA401") {
        info!("No RGB control available");
        // TODO: actual check for the AniMe device here
        laptop.support_animatrix = true;
    // GA502
    } else if board_name.starts_with("GA502")
        || board_name.starts_with("GU502")
    {
        info!("No RGB control available");
    // GX502, G712
    } else if board_name.starts_with("GX502")
        || board_name.starts_with("GX531")
        || board_name.starts_with("G532")
    {
        laptop.supported_modes = vec![
            SINGLE, BREATHING, STROBE, RAINBOW, STAR, RAIN, HIGHLIGHT, LASER, RIPPLE, PULSE, COMET,
            FLASH, RGB,
        ];
    // G512LI & G712LI has 1 RGB zone which means per-key effect might work
    } else if board_name.starts_with("G512LI") || board_name.starts_with("G712LI") {
        laptop.supported_modes = vec![SINGLE, BREATHING, STROBE, RAINBOW, PULSE];
    // GM501, GX531, G531, G512, G712 have 4-zone RGB
    } else if board_name.starts_with("GM501")
        || board_name.starts_with("G512")
        || board_name.starts_with("G712")
        || board_name.starts_with("G531")
    {
        laptop.supported_modes = vec![SINGLE, BREATHING, STROBE, RAINBOW, PULSE, MULTISTATIC];
    } else {
        warn!(
            "Unsupported laptop, please request support at {}",
            HELP_ADDRESS
        );
        warn!("Continuing with minimal support")
    }

    if !laptop.supported_modes.is_empty() {
        info!("Supported Keyboard LED modes are:");
        for mode in &laptop.supported_modes {
            let mode = <&str>::from(&<AuraModes>::from(*mode));
            info!("- {}", mode);
        }
        info!(
            "If these modes are incorrect or missing please request support at {}",
            HELP_ADDRESS
        );
    }

    laptop
}

pub(super) struct LaptopBase {
    usb_vendor: u16,
    usb_product: u16,
    report_filter_bytes: Vec<u8>,
    min_led_bright: u8,
    max_led_bright: u8,
    led_endpoint: u8,
    key_endpoint: u8,
    supported_modes: Vec<u8>,
    support_animatrix: bool,
}

use tokio::sync::{mpsc, Mutex};

impl LaptopBase {
    // Pass in LedWriter as Mutex so it is only locked when required
    /// Determines what action each fn+key combo should have
    pub(super) async fn do_keyboard_command(
        &self,
        rogcore: &mut RogCore,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
        mut aura_command: mpsc::Sender<AuraModes>,
    ) -> Result<(), AuraError> {
        let mut config = config.lock().await;
        match FnKeys::from(key_buf[1]) {
            FnKeys::LedBrightUp => {
                let mut bright = config.brightness;
                if bright < self.max_led_bright {
                    bright += 1;
                    info!("Increased LED brightness to {:#?}", bright);
                }
                aura_command
                    .send(AuraModes::LedBrightness(bright))
                    .await
                    .unwrap_or_else(|err| warn!("LedBrightUp: {}", err));
            }
            FnKeys::LedBrightDown => {
                let mut bright = config.brightness;
                if bright > self.min_led_bright {
                    bright -= 1;
                }
                aura_command
                    .send(AuraModes::LedBrightness(bright))
                    .await
                    .unwrap_or_else(|err| warn!("LedBrightDown: {}", err));
            }
            FnKeys::AuraNext => {
                if let Ok(idx) = self.supported_modes.binary_search(&config.current_mode) {
                    let idx_next = if idx < self.supported_modes.len() - 1 {
                        idx + 1
                    } else {
                        0
                    };
                    config.read();
                    if let Some(data) = config.get_led_mode_data(self.supported_modes[idx_next]) {
                        aura_command.send(data.clone()).await.unwrap_or_else(|_| {});
                    }
                } else {
                    warn!("Tried to step to next LED mode while in non-supported mode");
                }
            }
            FnKeys::AuraPrevious => {
                if let Ok(idx) = self.supported_modes.binary_search(&config.current_mode) {
                    let idx_next = if idx > 0 {
                        idx - 1
                    } else {
                        self.supported_modes.len() - 1
                    };
                    config.read();
                    if let Some(data) = config.get_led_mode_data(self.supported_modes[idx_next]) {
                        aura_command.send(data.clone()).await.unwrap_or_else(|_| {});
                    }
                } else {
                    warn!("Tried to step to next LED mode while in non-supported mode");
                }
            }
            FnKeys::ScreenBrightUp => rogcore.virt_keys().press(ConsumerKeys::BacklightInc.into()), //self.backlight.step_up(),
            FnKeys::ScreenBrightDn => rogcore.virt_keys().press(ConsumerKeys::BacklightDec.into()),
            FnKeys::ScreenToggle => {}
            FnKeys::Sleep => rogcore.suspend_with_systemd(),
            FnKeys::AirplaneMode => rogcore.toggle_airplane_mode(),
            FnKeys::MicToggle => rogcore.virt_keys().press(KeyboardKeys::MicToggle.into()),
            FnKeys::Fan => {
                rogcore.fan_mode_step(&mut config).unwrap_or_else(|err| {
                    warn!("Couldn't toggle fan mode: {:?}", err);
                });
            }
            FnKeys::TouchPadToggle => rogcore
                .virt_keys()
                .press(KeyboardKeys::TouchpadToggle.into()),
            FnKeys::Rog => rogcore.virt_keys().press(config.rog_key.into()),
            FnKeys::Calc => rogcore
                .virt_keys()
                .press(ConsumerKeys::LaunchCalculator.into()),
            FnKeys::None => {
                if key_buf[0] != 0x5A {
                    info!("Unmapped key, attempt passthrough: {:X?}", &key_buf[1]);
                    rogcore.virt_keys().press(key_buf);
                }
            }
        }
        Ok(())
    }

    pub(super) fn led_endpoint(&self) -> u8 {
        self.led_endpoint
    }
    pub(super) fn key_endpoint(&self) -> u8 {
        self.key_endpoint
    }
    pub(super) fn key_filter(&self) -> &[u8] {
        &self.report_filter_bytes
    }
    pub(super) fn usb_vendor(&self) -> u16 {
        self.usb_vendor
    }
    pub(super) fn usb_product(&self) -> u16 {
        self.usb_product
    }
    pub(super) fn supported_modes(&self) -> &[u8] {
        &self.supported_modes
    }
    pub(super) fn support_animatrix(&self) -> bool {
        self.support_animatrix
    }
    pub(super) fn set_support_animatrix(&mut self, supported: bool) {
        self.support_animatrix = supported;
    }
}
