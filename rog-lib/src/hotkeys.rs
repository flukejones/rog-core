use crate::aura::BuiltInModeByte;
use crate::core::RogCore;

pub trait Laptop {
    fn do_hotkey_action(&self, core: &mut RogCore, key_byte: u8);
    fn hotkey_group_byte(&self) -> u8;
    fn supported_modes(&self) -> &[BuiltInModeByte];
}
pub struct LaptopGX502GW {
    hotkey_group_byte: u8,
    min_bright: u8,
    max_bright: u8,
    supported_modes: Vec<BuiltInModeByte>,
}
impl LaptopGX502GW {
    pub fn new() -> Self {
        LaptopGX502GW {
            hotkey_group_byte: 0x5a,
            min_bright: 0x00,
            max_bright: 0x03,
            supported_modes: vec![
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
        }
    }
}
impl Laptop for LaptopGX502GW {
    fn do_hotkey_action(&self, rogcore: &mut RogCore, key_byte: u8) {
        match GX502GWKeys::from(key_byte) {
            GX502GWKeys::Rog => {
                println!("ROG!");
            }
            GX502GWKeys::LedBrightUp => {
                let mut bright = rogcore.config.brightness;
                if bright < self.max_bright {
                    bright += 1;
                    rogcore.config.brightness = bright;
                }
                let bytes = RogCore::aura_brightness_bytes(bright).unwrap();
                rogcore
                    .aura_set_and_save(&bytes, &self.supported_modes)
                    .unwrap();
            }
            GX502GWKeys::LedBrightDown => {
                let mut bright = rogcore.config.brightness;
                if bright > self.min_bright {
                    bright -= 1;
                    rogcore.config.brightness = bright;
                }
                let bytes = RogCore::aura_brightness_bytes(bright).unwrap();
                rogcore
                    .aura_set_and_save(&bytes, &self.supported_modes)
                    .unwrap();
            }
            GX502GWKeys::AuraNext => {
                let mut mode = rogcore.config.current_mode[3] + 1;
                if mode > 0x0c {
                    mode = 0x00
                } else if mode == 0x09 {
                    mode = 0x0a
                }
                rogcore.config.current_mode[3] = mode;
                if let Some(bytes) = rogcore.config.get_current() {
                    rogcore
                        .aura_set_and_save(&bytes, &self.supported_modes)
                        .unwrap();
                }
            }
            GX502GWKeys::AuraPrevious => {
                let mut mode = rogcore.config.current_mode[3];
                if mode == 0x00 {
                    mode = 0x0c
                } else if mode - 1 == 0x09 {
                    mode = 0x08
                } else {
                    mode -= 1;
                }
                rogcore.config.current_mode[3] = mode;
                if let Some(bytes) = rogcore.config.get_current() {
                    rogcore
                        .aura_set_and_save(&bytes, &self.supported_modes)
                        .unwrap();
                    rogcore.config.write();
                }
            }
            _ => {}
        }
    }
    fn hotkey_group_byte(&self) -> u8 {
        self.hotkey_group_byte
    }

    fn supported_modes(&self) -> &[BuiltInModeByte] {
        &self.supported_modes
    }
}

pub enum GX502GWKeys {
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
