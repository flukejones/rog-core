use crate::aura::BuiltInModeByte;
use crate::core::{Backlight, RogCore};
use crate::error::AuraError;
use crate::virt_device::ConsumerKeys;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use super::Laptop;
use log::info;

pub(super) struct LaptopGL753 {
    usb_vendor: u16,
    usb_product: u16,
    board_name: &'static str,
    prod_family: &'static str,
    report_filter_bytes: [u8; 2],
    min_led_bright: u8,
    max_led_bright: u8,
    led_endpoint: u8,
    key_endpoint: u8,
    supported_modes: [BuiltInModeByte; 3],
    backlight: Backlight,
}

impl LaptopGL753 {
    pub(super) fn new() -> Self {
        LaptopGL753 {
            usb_vendor: 0x0B05,
            usb_product: 0x1854,
            // from `cat /sys/class/dmi/id/board_name`
            board_name: "LaptopGL753VE",
            // from `cat /sys/class/dmi/id/product_family`
            prod_family: "",
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
            ],
            backlight: Backlight::new("intel_backlight").unwrap(),
        }
    }
}

impl LaptopGL753 {
    fn do_keypress_actions(&self, rogcore: &mut RogCore) -> Result<(), AuraError> {
        if let Some(key_buf) = rogcore.poll_keyboard(&self.report_filter_bytes) {
            match GL753Keys::from(key_buf[1]) {
                GL753Keys::LedBrightUp => {
                    rogcore.aura_bright_inc(&self.supported_modes, self.max_led_bright)?;
                }
                GL753Keys::LedBrightDown => {
                    rogcore.aura_bright_dec(&self.supported_modes, self.min_led_bright)?;
                }
                GL753Keys::ScreenBrightUp => self.backlight.step_up(),
                GL753Keys::ScreenBrightDown => self.backlight.step_down(),
                GL753Keys::Sleep => rogcore.suspend_with_systemd(),
                GL753Keys::AirplaneMode => rogcore.toggle_airplane_mode(),
                GL753Keys::ScreenToggle => {
                    rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
                }
                GL753Keys::TouchPadToggle => {
                    let mut key = [0u8; 32];
                    key[0] = 0x01;
                    key[3] = 0x070;
                    rogcore.virt_keys().press(key);
                }
                GL753Keys::Rog => {
                    let mut key = [0u8; 32];
                    key[0] = 0x01;
                    key[3] = 0x68; // XF86Tools? F13
                    rogcore.virt_keys().press(key);
                }
                GL753Keys::None => {
                    if key_buf[0] != 0x5A {
                        info!("Unmapped key array: {:X?}", &key_buf);
                        info!("Attempting passthrough: {:X?}", &key_buf[1]);
                        rogcore.virt_keys().press(key_buf);
                    }
                }
            }
        }
        Ok(())
    }
}

impl Laptop for LaptopGL753 {
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

enum GL753Keys {
    Rog = 0x38,
    ScreenToggle = 0x35,
    ScreenBrightDown = 0x10,
    ScreenBrightUp = 0x20,
    TouchPadToggle = 0x6b,
    Sleep = 0x6C,
    AirplaneMode = 0x88,
    LedBrightUp = 0xC4,
    LedBrightDown = 0xC5,
    None,
}

impl From<u8> for GL753Keys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => GL753Keys::Rog,
            0x35 => GL753Keys::ScreenToggle,
            0x10 => GL753Keys::ScreenBrightDown,
            0x20 => GL753Keys::ScreenBrightUp,
            0x6b => GL753Keys::TouchPadToggle,
            0x6C => GL753Keys::Sleep,
            0x88 => GL753Keys::AirplaneMode,
            0xC4 => GL753Keys::LedBrightUp,
            0xC5 => GL753Keys::LedBrightDown,
            _ => GL753Keys::None,
        }
    }
}
