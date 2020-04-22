use crate::aura::{BuiltInModeByte, Key, KeyColourArray};
use crate::core::{Backlight, RogCore};
use crate::error::AuraError;
use crate::virt_device::ConsumerKeys;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use super::Laptop;
use log::info;

pub(super) struct LaptopGX502GW {
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

impl LaptopGX502GW {
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

        for _ in 0..3 {
            for _ in 0..50 {
                *key_colours.key(Key::ROG).0 -= 5;
                *key_colours.key(Key::L).0 -= 5;
                *key_colours.key(Key::I).0 -= 5;
                *key_colours.key(Key::N).0 -= 5;
                *key_colours.key(Key::U).0 -= 5;
                *key_colours.key(Key::X).0 -= 5;
                per_key_led.push(key_colours.clone());
            }
            for _ in 0..50 {
                *key_colours.key(Key::ROG).0 += 5;
                *key_colours.key(Key::L).0 += 5;
                *key_colours.key(Key::I).0 += 5;
                *key_colours.key(Key::N).0 += 5;
                *key_colours.key(Key::U).0 += 5;
                *key_colours.key(Key::X).0 += 5;
                per_key_led.push(key_colours.clone());
            }
        }

        // Find backlight
        LaptopGX502GW {
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
                BuiltInModeByte::Stable,
                BuiltInModeByte::Breathe,
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

impl LaptopGX502GW {
    fn do_keypress_actions(&self, rogcore: &mut RogCore) -> Result<(), AuraError> {
        if let Some(key_buf) = rogcore.poll_keyboard(&self.report_filter_bytes) {
            match GX502GWKeys::from(key_buf[1]) {
                GX502GWKeys::LedBrightUp => {
                    rogcore.aura_bright_inc(&self.supported_modes, self.max_led_bright)?;
                }
                GX502GWKeys::LedBrightDown => {
                    rogcore.aura_bright_dec(&self.supported_modes, self.min_led_bright)?;
                }
                GX502GWKeys::AuraNext => rogcore.aura_mode_next(&self.supported_modes)?,
                GX502GWKeys::AuraPrevious => rogcore.aura_mode_prev(&self.supported_modes)?,
                GX502GWKeys::ScreenBrightUp => self.backlight.step_up(),
                GX502GWKeys::ScreenBrightDown => self.backlight.step_down(),
                GX502GWKeys::Sleep => rogcore.suspend_with_systemd(),
                GX502GWKeys::AirplaneMode => rogcore.toggle_airplane_mode(),
                GX502GWKeys::MicToggle => {}
                GX502GWKeys::Fan => {}
                GX502GWKeys::ScreenToggle => {
                    rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
                }
                GX502GWKeys::TouchPadToggle => {
                    let mut key = [0u8; 32];
                    key[0] = 0x01;
                    key[3] = 0x070;
                    rogcore.virt_keys().press(key);
                }
                GX502GWKeys::Rog => {
                    rogcore.aura_effect_init()?;
                    rogcore.aura_write_effect(&self.per_key_led)?;
                    // let mut key = [0u8; 32];
                    // key[0] = 0x01;
                    // key[3] = 0x68; // XF86Tools? F13
                    // rogcore.virt_keys().press(key);
                }
                GX502GWKeys::None => {
                    if key_buf[0] != 0x5A {
                        info!("Unmapped key, attempt passthrough: {:X?}", &key_buf[1]);
                        rogcore.virt_keys().press(key_buf);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Laptop for LaptopGX502GW {
    fn run(&self, rogcore: &mut RogCore) -> Result<(), AuraError> {
        self.do_keypress_actions(rogcore)
    }
    fn led_endpoint(&self) -> u8 {
        self.led_endpoint
    }
    fn key_endpoint(&self) -> u8 {
        self.key_endpoint
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

enum GX502GWKeys {
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

impl From<u8> for GX502GWKeys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => GX502GWKeys::Rog,
            0x7C => GX502GWKeys::MicToggle,
            0xAE => GX502GWKeys::Fan,
            0x35 => GX502GWKeys::ScreenToggle,
            0x10 => GX502GWKeys::ScreenBrightDown,
            0x20 => GX502GWKeys::ScreenBrightUp,
            0x6b => GX502GWKeys::TouchPadToggle,
            0x6C => GX502GWKeys::Sleep,
            0x88 => GX502GWKeys::AirplaneMode,
            0xC4 => GX502GWKeys::LedBrightUp,
            0xC5 => GX502GWKeys::LedBrightDown,
            0xB2 => GX502GWKeys::AuraPrevious,
            0xB3 => GX502GWKeys::AuraNext,
            _ => GX502GWKeys::None,
        }
    }
}
