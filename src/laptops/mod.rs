use crate::aura::BuiltInModeByte;
use crate::core::RogCore;
use crate::error::AuraError;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use log::info;

mod gx502gw;
use gx502gw::LaptopGX502GW;

pub(crate) fn match_laptop() -> Box<dyn Laptop> {
    let dmi = sysfs_class::DmiId::default();
    let board_name = dmi.board_name().unwrap();
    match board_name.as_str() {
        // The hell does it have a \n for anyway?
        "GX502GW\n" => {
            info!("Found GX502GW");
            Box::new(LaptopGX502GW::new())
        }
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
