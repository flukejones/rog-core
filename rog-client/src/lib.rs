pub static DBUS_NAME: &str = "org.rogcore.Daemon";
pub static DBUS_PATH: &str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &str = "org.rogcore.Daemon";
pub const LED_MSG_LEN: usize = 17;

mod builtins;
pub use builtins::*;

/// Contains mostly only what is required for parsing CLI options
pub mod cli_options;
mod fancy;

mod aura_dbus;
pub use aura_dbus::*;

pub use fancy::*;

mod animatrix_dbus;
pub use animatrix_dbus::*;

mod anime_matrix;
pub use anime_matrix::*;

pub mod error;
use crate::cli_options::*;

/// Writes aout the correct byte string for brightness
///
/// The HID descriptor looks like:
///
/// ```
/// 0x06, 0x31, 0xFF,  // Usage Page (Vendor Defined 0xFF31)
/// 0x09, 0x76,        // Usage (0x76)
/// 0xA1, 0x01,        // Collection (Application)
/// 0x85, 0x5A,        //   Report ID (90)
/// 0x19, 0x00,        //   Usage Minimum (0x00)
/// 0x2A, 0xFF, 0x00,  //   Usage Maximum (0xFF)
/// 0x15, 0x00,        //   Logical Minimum (0)
/// 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
/// 0x75, 0x08,        //   Report Size (8)
/// 0x95, 0x05,        //   Report Count (5)
/// 0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
/// 0x19, 0x00,        //   Usage Minimum (0x00)
/// 0x2A, 0xFF, 0x00,  //   Usage Maximum (0xFF)
/// 0x15, 0x00,        //   Logical Minimum (0)
/// 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
/// 0x75, 0x08,        //   Report Size (8)
/// 0x95, 0x3F,        //   Report Count (63)
/// 0xB1, 0x00,        //   Feature (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
/// 0xC0,              // End Collection
/// ```
pub fn aura_brightness_bytes(brightness: u8) -> [u8; 17] {
    // TODO: check brightness range
    [
        0x5A, 0xBA, 0xC5, 0xC4, brightness, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Parses `SetAuraBuiltin` in to packet data
///
/// Byte structure:
///
/// ```
/// | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11| 12|
/// |---|---|---|---|---|---|---|---|---|---|---|---|---|
/// |5d |b3 |00 |03 |ff |00 |00 |00 |00 |00 |00 |ff |00 |
/// ```
///
/// Bytes 0 and 1 should always be 5d, b3
///
/// On multizone laptops byte 2 is the zone number, RGB in usual
/// place, byte 3 set to zero
///
/// Byte 3 sets the mode type:
/// - 00 = static
/// - 01 = breathe (can set two colours)
/// - 02 = strobe (through all colours)
/// - 03 = rainbow
/// - 04 = star (byte 9 sets rain colour)
/// - 05 = rain keys, red, white, turquoise
/// - 06 = pressed keys light up and fade
/// - 07 = pressed key emits laser
/// - 08 = pressed key emits water ripple
/// - 09 = no effect/not used
/// - 0a fast pulse (no speed setting)
/// - 0b vertical line racing to right (no speed setting)
/// - 0c wider vertical line racing to right (no speed setting)
///
/// Bytes 4, 5, 6 are Red, Green, Blue
///
/// Byte 7 sets speed from
/// - 0x00 = Off
/// - 0xe1 = Slow
/// - 0xeb = Medium
/// - 0xf5 = Fast
///
/// Byte 8 sets rainbow direction:
/// - 0x00 = rightwards
/// - 0x01 = leftwards
/// - 0x02 = upwards
/// - 0x03 = downwards
///
/// Bytes 10, 11, 12 are Red, Green, Blue for second colour if mode supports it
///
/// The HID descriptor looks like:
/// ```
/// 0x06, 0x31, 0xFF,  // Usage Page (Vendor Defined 0xFF31)
/// 0x09, 0x79,        // Usage (0x79)
/// 0xA1, 0x01,        // Collection (Application)
/// 0x85, 0x5D,        //   Report ID (93)
/// 0x19, 0x00,        //   Usage Minimum (0x00)
/// 0x2A, 0xFF, 0x00,  //   Usage Maximum (0xFF)
/// 0x15, 0x00,        //   Logical Minimum (0)
/// 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
/// 0x75, 0x08,        //   Report Size (8)
/// 0x95, 0x1F,        //   Report Count (31)
/// 0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
/// 0x19, 0x00,        //   Usage Minimum (0x00)
/// 0x2A, 0xFF, 0x00,  //   Usage Maximum (0xFF)
/// 0x15, 0x00,        //   Logical Minimum (0)
/// 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
/// 0x75, 0x08,        //   Report Size (8)
/// 0x95, 0x3F,        //   Report Count (63)
/// 0x91, 0x00,        //   Output (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
/// 0x19, 0x00,        //   Usage Minimum (0x00)
/// 0x2A, 0xFF, 0x00,  //   Usage Maximum (0xFF)
/// 0x15, 0x00,        //   Logical Minimum (0)
/// 0x26, 0xFF, 0x00,  //   Logical Maximum (255)
/// 0x75, 0x08,        //   Report Size (8)
/// 0x95, 0x3F,        //   Report Count (63)
/// 0xB1, 0x00,        //   Feature (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
/// 0xC0,              // End Collection
/// ```
///
/// This descriptor is also used for the per-key LED settings
impl From<&SetAuraBuiltin> for [u8; LED_MSG_LEN] {
    fn from(mode: &SetAuraBuiltin) -> Self {
        let mut msg = [0u8; LED_MSG_LEN];
        msg[0] = 0x5d;
        msg[1] = 0xb3;
        match mode {
            SetAuraBuiltin::Stable(_) => msg[3] = 0x00,
            SetAuraBuiltin::Breathe(_) => msg[3] = 0x01,
            SetAuraBuiltin::Strobe(_) => msg[3] = 0x02,
            SetAuraBuiltin::Rainbow(_) => msg[3] = 0x03,
            SetAuraBuiltin::Star(_) => msg[3] = 0x04,
            SetAuraBuiltin::Rain(_) => msg[3] = 0x05,
            SetAuraBuiltin::Highlight(_) => msg[3] = 0x06,
            SetAuraBuiltin::Laser(_) => msg[3] = 0x07,
            SetAuraBuiltin::Ripple(_) => msg[3] = 0x08,
            SetAuraBuiltin::Pulse(_) => msg[3] = 0x0a,
            SetAuraBuiltin::Comet(_) => msg[3] = 0x0b,
            SetAuraBuiltin::Flash(_) => msg[3] = 0x0c,
            _ => panic!("Mode not convertable to array"),
        }

        match mode {
            SetAuraBuiltin::Rainbow(settings) => {
                msg[7] = settings.speed as u8;
                msg[8] = settings.direction as u8;
            }
            SetAuraBuiltin::Star(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
                msg[7] = settings.speed as u8;
                msg[9] = settings.colour2.2;
            }
            SetAuraBuiltin::Breathe(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
                msg[7] = settings.speed as u8;
                msg[10] = settings.colour2.0;
                msg[11] = settings.colour2.1;
                msg[12] = settings.colour2.2;
            }
            SetAuraBuiltin::Strobe(settings) | SetAuraBuiltin::Rain(settings) => {
                msg[7] = settings.speed as u8;
            }
            SetAuraBuiltin::Highlight(settings)
            | SetAuraBuiltin::Laser(settings)
            | SetAuraBuiltin::Ripple(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
                msg[7] = settings.speed as u8;
            }
            SetAuraBuiltin::Stable(settings)
            | SetAuraBuiltin::Pulse(settings)
            | SetAuraBuiltin::Comet(settings)
            | SetAuraBuiltin::Flash(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
            }
            _ => panic!("Mode not convertable to array"),
        }
        msg
    }
}

impl From<SetAuraBuiltin> for [u8; LED_MSG_LEN] {
    #[inline]
    fn from(mode: SetAuraBuiltin) -> Self {
        <[u8; LED_MSG_LEN]>::from(&mode)
    }
}

impl From<SetAuraBuiltin> for [[u8; LED_MSG_LEN]; 4] {
    #[inline]
    fn from(mode: SetAuraBuiltin) -> Self {
        let mut msg = [[0u8; LED_MSG_LEN]; 4];
        for i in 0..4 {
            msg[i][0] = 0x5d;
            msg[i][1] = 0xb3;
            msg[i][2] = i as u8 + 1;
        }

        match mode {
            SetAuraBuiltin::MultiStatic(settings) => {
                msg[0][4] = settings.colour1.0;
                msg[0][5] = settings.colour1.1;
                msg[0][6] = settings.colour1.2;
                msg[1][4] = settings.colour2.0;
                msg[1][5] = settings.colour2.1;
                msg[1][6] = settings.colour2.2;
                msg[2][4] = settings.colour3.0;
                msg[2][5] = settings.colour3.1;
                msg[2][6] = settings.colour3.2;
                msg[3][4] = settings.colour4.0;
                msg[3][5] = settings.colour4.1;
                msg[3][6] = settings.colour4.2;
            }
            _ => panic!("Mode not convertable to array"),
        }
        msg
    }
}
