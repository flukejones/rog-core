use crate::aura::{BuiltInModeByte, Key, KeyColourArray};
use crate::core::{Backlight, RogCore};
use crate::error::AuraError;
use crate::virt_device::ConsumerKeys;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use super::Laptop;
use log::{info, warn};

pub(super) struct LaptopGX502 {
    usb_vendor: u16,
    usb_product: u16,
    board_name: &'static str,
    prod_family: &'static str,
    report_filter_bytes: [u8; 2],
    min_led_bright: u8,
    max_led_bright: u8,
    led_endpoint: u8,
    key_endpoint: u8,
    supported_modes: [BuiltInModeByte; 12],
    backlight: Backlight,
    per_key_led: Vec<KeyColourArray>,
}

impl LaptopGX502 {
    pub(super) fn new() -> Self {
        // Setting up a sample effect to run when ROG pressed
        let mut per_key_led = Vec::new();
        let mut key_colours = KeyColourArray::new();
        key_colours.set(Key::ROG, 255, 0, 0);
        key_colours.set(Key::L, 255, 0, 0);
        key_colours.set(Key::I, 255, 0, 0);
        key_colours.set(Key::N, 255, 0, 0);
        key_colours.set(Key::U, 255, 0, 0);
        key_colours.set(Key::X, 255, 0, 0);
        per_key_led.push(key_colours.clone());

        for _ in 0..49 {
            *key_colours.key(Key::ROG).0 -= 5;
            *key_colours.key(Key::L).0 -= 5;
            *key_colours.key(Key::I).0 -= 5;
            *key_colours.key(Key::N).0 -= 5;
            *key_colours.key(Key::U).0 -= 5;
            *key_colours.key(Key::X).0 -= 5;
            per_key_led.push(key_colours.clone());
        }
        for _ in 0..49 {
            *key_colours.key(Key::ROG).0 += 5;
            *key_colours.key(Key::L).0 += 5;
            *key_colours.key(Key::I).0 += 5;
            *key_colours.key(Key::N).0 += 5;
            *key_colours.key(Key::U).0 += 5;
            *key_colours.key(Key::X).0 += 5;
            per_key_led.push(key_colours.clone());
        }

        // Find backlight
        LaptopGX502 {
            usb_vendor: 0x0B05,
            usb_product: 0x1866,
            // from `cat /sys/class/dmi/id/board_name`
            board_name: "GX502GW",
            // from `cat /sys/class/dmi/id/product_family`
            prod_family: "Zephyrus S",
            report_filter_bytes: [0x5a, 0x02],
            min_led_bright: 0x00,
            max_led_bright: 0x03,
            //from `lsusb -vd 0b05:1866`
            led_endpoint: 0x04,
            //from `lsusb -vd 0b05:1866`
            key_endpoint: 0x83,
            supported_modes: [
                BuiltInModeByte::Single,
                BuiltInModeByte::Breathing,
                BuiltInModeByte::Cycle,
                BuiltInModeByte::Rainbow,
                BuiltInModeByte::Rain,
                BuiltInModeByte::Random,
                BuiltInModeByte::Highlight,
                BuiltInModeByte::Laser,
                BuiltInModeByte::Ripple,
                BuiltInModeByte::Pulse,
                BuiltInModeByte::ThinZoomy,
                BuiltInModeByte::WideZoomy,
            ],
            backlight: Backlight::new("intel_backlight").unwrap(),
            per_key_led,
        }
    }
}

impl LaptopGX502 {
    fn do_keypress_actions(
        &self,
        rogcore: &mut RogCore,
        key_buf: [u8; 32],
    ) -> Result<(), AuraError> {
        match GX502Keys::from(key_buf[1]) {
            GX502Keys::LedBrightUp => {
                rogcore.aura_bright_inc(&self.supported_modes, self.max_led_bright)?;
            }
            GX502Keys::LedBrightDown => {
                rogcore.aura_bright_dec(&self.supported_modes, self.min_led_bright)?;
            }
            GX502Keys::AuraNext => rogcore.aura_mode_next(&self.supported_modes)?,
            GX502Keys::AuraPrevious => rogcore.aura_mode_prev(&self.supported_modes)?,
            GX502Keys::ScreenBrightUp => self.backlight.step_up(),
            GX502Keys::ScreenBrightDown => self.backlight.step_down(),
            GX502Keys::Sleep => rogcore.suspend_with_systemd(),
            GX502Keys::AirplaneMode => rogcore.toggle_airplane_mode(),
            GX502Keys::MicToggle => {}
            GX502Keys::Fan => {
                rogcore.fan_mode_step().unwrap_or_else(|err| {
                    warn!("Couldn't toggle fan mode: {:?}", err);
                });
            }
            GX502Keys::ScreenToggle => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
            }
            GX502Keys::TouchPadToggle => {
                let mut key = [0u8; 32];
                key[0] = 0x01;
                key[3] = 0x070;
                rogcore.virt_keys().press(key);
            }
            GX502Keys::Rog => {
                //rogcore.aura_effect_init()?;
                //rogcore.aura_write_effect(&self.per_key_led)?;
                // let mut key = [0u8; 32];
                // key[0] = 0x01;
                // key[3] = 0x68; // XF86Tools? F13
                // rogcore.virt_keys().press(key);
            }
            GX502Keys::None => {
                if key_buf[0] != 0x5A {
                    info!("Unmapped key, attempt passthrough: {:X?}", &key_buf[1]);
                    rogcore.virt_keys().press(key_buf);
                }
            }
        }
        Ok(())
    }
}

impl Laptop for LaptopGX502 {
    fn run(&self, rogcore: &mut RogCore, key_buf: [u8; 32]) -> Result<(), AuraError> {
        self.do_keypress_actions(rogcore, key_buf)
    }
    fn led_endpoint(&self) -> u8 {
        self.led_endpoint
    }
    fn key_endpoint(&self) -> u8 {
        self.key_endpoint
    }
    fn key_filter(&self) -> &[u8] {
        &self.report_filter_bytes
    }
    fn usb_vendor(&self) -> u16 {
        self.usb_vendor
    }
    fn usb_product(&self) -> u16 {
        self.usb_product
    }
    fn supported_modes(&self) -> &[BuiltInModeByte] {
        &self.supported_modes
    }
    fn board_name(&self) -> &str {
        &self.board_name
    }
    fn prod_family(&self) -> &str {
        &self.prod_family
    }
}

enum GX502Keys {
    Rog = 0x38,
    MicToggle = 0x7C,
    Fan = 0xAE,
    ScreenToggle = 0x35,
    ScreenBrightDown = 0x10,
    ScreenBrightUp = 0x20,
    TouchPadToggle = 0x6b,
    Sleep = 0x6C,
    AirplaneMode = 0x88,
    LedBrightUp = 0xC4,
    LedBrightDown = 0xC5,
    AuraPrevious = 0xB2,
    AuraNext = 0xB3,
    None,
}

impl From<u8> for GX502Keys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => GX502Keys::Rog,
            0x7C => GX502Keys::MicToggle,
            0xAE => GX502Keys::Fan,
            0x35 => GX502Keys::ScreenToggle,
            0x10 => GX502Keys::ScreenBrightDown,
            0x20 => GX502Keys::ScreenBrightUp,
            0x6b => GX502Keys::TouchPadToggle,
            0x6C => GX502Keys::Sleep,
            0x88 => GX502Keys::AirplaneMode,
            0xC4 => GX502Keys::LedBrightUp,
            0xC5 => GX502Keys::LedBrightDown,
            0xB2 => GX502Keys::AuraPrevious,
            0xB3 => GX502Keys::AuraNext,
            _ => GX502Keys::None,
        }
    }
}
