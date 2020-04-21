use crate::aura::BuiltInModeByte;
use crate::core::{Backlight, RogCore};
use crate::error::AuraError;
use crate::virt_device::ConsumerKeys;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use log::info;

pub fn match_laptop() -> Box<dyn LaptopRunner> {
    let dmi = sysfs_class::DmiId::default();
    let board_name = dmi.board_name().unwrap();
    match board_name.as_str() {
        // The hell does it have a \n for anyway?
        "GX502GW\n" => Box::new(LaptopGX502GW::new()),
        _ => {
            panic!("could not match laptop");
        }
    }
}

/// All laptop models should implement this trait. The role of a `Laptop` is to
/// "drive" the `RogCore`.
///
/// `do_hotkey_action` is passed the byte that a hotkey emits, and is expected to
/// perform whichever action matches that. For now the only key bytes passed in are
/// the ones which match `byte[0] == hotkey_group_byte`. On the GX502GW the keyboard
/// has 3 explicit groups: main, vol+media, and the ones that the Linux kernel doesn't
/// map.
pub(crate) trait Laptop {
    fn hotkey_group_bytes(&self) -> &[u8];
    fn board_name(&self) -> &str;
    fn prod_family(&self) -> &str;
}

/// The public interface for running a laptop. Primarily used by the daemon.
pub trait LaptopRunner {
    fn run(&self, core: &mut RogCore) -> Result<(), AuraError>;
    fn led_iface_num(&self) -> u8;
    fn key_iface_num(&self) -> u8;
    fn usb_vendor(&self) -> u16;
    fn usb_product(&self) -> u16;
    fn supported_modes(&self) -> &[BuiltInModeByte];
}

pub(crate) struct LaptopGX502GW {
    usb_vendor: u16,
    usb_product: u16,
    board_name: &'static str,
    prod_family: &'static str,
    hotkey_group_bytes: [u8; 2],
    min_led_bright: u8,
    max_led_bright: u8,
    led_iface_num: u8,
    key_iface_num: u8,
    supported_modes: [BuiltInModeByte; 12],
    backlight: Backlight,
}

impl LaptopGX502GW {
    pub fn new() -> Self {
        // Find backlight
        LaptopGX502GW {
            usb_vendor: 0x0B05,
            usb_product: 0x1866,
            board_name: "GX502GW",
            prod_family: "Zephyrus S",
            hotkey_group_bytes: [0x5a, 0x02],
            min_led_bright: 0x00,
            max_led_bright: 0x03,
            led_iface_num: 0x81,
            key_iface_num: 0x83,
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
        }
    }
}

impl LaptopRunner for LaptopGX502GW {
    // TODO: This really needs to match against u16 in future
    fn run(&self, rogcore: &mut RogCore) -> Result<(), AuraError> {
        let mut key_buf = [0u8; 32];
        rogcore.poll_keyboard(&self.hotkey_group_bytes, &mut key_buf)?;

        match GX502GWKeys::from(key_buf[1]) {
            GX502GWKeys::LedBrightUp => {
                let mut bright = rogcore.config().brightness;
                if bright < self.max_led_bright {
                    bright += 1;
                    rogcore.config_mut().brightness = bright;
                }
                let bytes = RogCore::aura_brightness_bytes(bright)?;
                rogcore.aura_set_and_save(&self.supported_modes, &bytes)?;
            }
            GX502GWKeys::LedBrightDown => {
                let mut bright = rogcore.config().brightness;
                if bright > self.min_led_bright {
                    bright -= 1;
                    rogcore.config_mut().brightness = bright;
                }
                let bytes = RogCore::aura_brightness_bytes(bright)?;
                rogcore.aura_set_and_save(&self.supported_modes, &bytes)?;
            }
            GX502GWKeys::AuraNext => {
                let mut mode = rogcore.config().current_mode[3] + 1;
                if mode > 0x0c {
                    mode = 0x00
                } else if mode == 0x09 {
                    mode = 0x0a
                }
                rogcore.config_mut().current_mode[3] = mode;
                if let Some(bytes) = rogcore.config_mut().get_current() {
                    rogcore.aura_set_and_save(&self.supported_modes, &bytes)?;
                }
            }
            GX502GWKeys::AuraPrevious => {
                let mut mode = rogcore.config().current_mode[3];
                if mode == 0x00 {
                    mode = 0x0c
                } else if mode - 1 == 0x09 {
                    mode = 0x08
                } else {
                    mode -= 1;
                }
                rogcore.config_mut().current_mode[3] = mode;
                if let Some(bytes) = rogcore.config_mut().get_current() {
                    rogcore.aura_set_and_save(&self.supported_modes, &bytes)?;
                    rogcore.config().write();
                }
            }
            GX502GWKeys::ScreenBrightUp => {
                self.backlight.step_up();
            }
            GX502GWKeys::ScreenBrightDown => {
                self.backlight.step_down();
            }
            GX502GWKeys::Sleep => {
                // Direct call to systemd
                rogcore.suspend_with_systemd();
                //rogcore.virt_keys().press([0x01, 0, 0, 0x82, 0, 0, 0, 0]);
                // Power menu
                //rogcore.virt_keys().press([0x01, 0, 0, 0x66, 0, 0, 0, 0]);
            }
            GX502GWKeys::AirplaneMode => {
                rogcore.toggle_airplane_mode();
            }

            GX502GWKeys::MicToggle => {}
            GX502GWKeys::Fan => {}
            GX502GWKeys::ScreenToggle => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
            }
            GX502GWKeys::TouchPadToggle => {
                // F21 key, Touchpad toggle
                rogcore.virt_keys().press([0x01, 0, 0, 0x70, 0, 0, 0, 0]);
                // rogcore.virt_keys().press([0x01, 0, 0, 0x71, 0, 0, 0, 0]); // Touchpad on F22
                // rogcore.virt_keys().press([0x01, 0, 0, 0x72, 0, 0, 0, 0]); // Touchpad off F23
            }
            GX502GWKeys::Rog => {
                rogcore.virt_keys().press([0x01, 0, 0, 0x68, 0, 0, 0, 0]); // XF86Tools? F13
            }

            GX502GWKeys::None => {
                if key_buf[1] != 0 {
                    info!(
                        "Unmapped key, attempt to pass to virtual device: {:X?}",
                        &key_buf[1]
                    );
                    let mut bytes = [0u8; 8];
                    // TODO: code page
                    bytes[0] = 0x02;
                    bytes[1] = key_buf[1];
                    rogcore.virt_keys().press(bytes);
                }
            }
        }
        Ok(())
    }

    fn led_iface_num(&self) -> u8 {
        self.led_iface_num
    }
    fn key_iface_num(&self) -> u8 {
        self.key_iface_num
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
}

impl Laptop for LaptopGX502GW {
    fn hotkey_group_bytes(&self) -> &[u8] {
        &self.hotkey_group_bytes
    }
    fn board_name(&self) -> &str {
        &self.board_name
    }
    fn prod_family(&self) -> &str {
        &self.prod_family
    }
}

pub(crate) enum GX502GWKeys {
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
