use crate::aura::BuiltInModeByte;
use crate::core::RogCore;
use crate::error::AuraError;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use log::info;

// GL753VE == 0x1854, 4 zone keyboard
mod gl753;
use gl753::LaptopGL753;

// 0x1866, per-key LEDs, media-keys split from vendor specific
mod gx502;
use gx502::LaptopGX502;

pub(crate) fn match_laptop() -> Box<dyn Laptop> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0b05 {
            match device_desc.product_id() {
                0x1866 => {
                    info!("Found GX502 or similar");
                    return Box::new(LaptopGX502::new());
                }
                0x1854 => {
                    info!("Found GL753 or similar");
                    return Box::new(LaptopGL753::new());
                }
                _ => {}
            }
        }
    }
    panic!("could not match laptop");

    // let dmi = sysfs_class::DmiId::default();
    // let board_name = dmi.board_name().unwrap();
    // match &board_name.as_str()[..5] {
    //     // TODO: FX503VD - 0x1869,
    //     // These two models seem to have the same characteristics
    //     "GX502" | "GA502" | "GU502" => {
    //         info!("Found GX502 or GA502 series");
    //         Box::new(LaptopGX502::new())
    //     }
    //     // LED should work for this, but unsure of keys
    //     "GL753" => {
    //         info!("Found GL753 series");
    //         Box::new(LaptopGL753::new())
    //     }
    //     _ => {
    //         panic!("could not match laptop");
    //     }
    // }
}

/// All laptop models should implement this trait. The role of a `Laptop` is to
/// "drive" the `RogCore`.
///
/// `do_hotkey_action` is passed the byte that a hotkey emits, and is expected to
/// perform whichever action matches that. For now the only key bytes passed in are
/// the ones which match `byte[0] == hotkey_group_byte`. On the GX502GW the keyboard
/// has 3 explicit groups: main, vol+media, and the ones that the Linux kernel doesn't
/// map.
///
/// If using the `keycode` crate to build keyboard input, the report must be prefixed
/// with the report ID (usually `0x01` for the virtual keyboard).
pub(crate) trait Laptop {
    fn board_name(&self) -> &str;
    fn prod_family(&self) -> &str;
    fn run(&self, core: &mut RogCore) -> Result<(), AuraError>;
    fn led_endpoint(&self) -> u8;
    fn key_endpoint(&self) -> u8;
    fn usb_vendor(&self) -> u16;
    fn usb_product(&self) -> u16;
    fn supported_modes(&self) -> &[BuiltInModeByte];
}
