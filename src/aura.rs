use crate::cli_options::*;
use crate::core::LED_MSG_LEN;
use serde_derive::{Deserialize, Serialize};

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
/// - 02 = cycle (through all colours)
/// - 03 = rainbow
/// - 04 = rain
/// - 05 = random keys, red, white, turquoise
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
impl From<SetAuraBuiltin> for [u8; LED_MSG_LEN] {
    fn from(mode: SetAuraBuiltin) -> Self {
        let mut msg = [0u8; LED_MSG_LEN];
        msg[0] = 0x5d;
        msg[1] = 0xb3;
        match mode {
            SetAuraBuiltin::Stable(_) => msg[3] = 0x00,
            SetAuraBuiltin::Breathe(_) => msg[3] = 0x01,
            SetAuraBuiltin::Cycle(_) => msg[3] = 0x02,
            SetAuraBuiltin::Rainbow(_) => msg[3] = 0x03,
            SetAuraBuiltin::Rain(_) => msg[3] = 0x04,
            SetAuraBuiltin::Disco(_) => msg[3] = 0x05,
            SetAuraBuiltin::Highlight(_) => msg[3] = 0x06,
            SetAuraBuiltin::Laser(_) => msg[3] = 0x07,
            SetAuraBuiltin::Ripple(_) => msg[3] = 0x08,
            SetAuraBuiltin::Pulse(_) => msg[3] = 0x0a,
            SetAuraBuiltin::ThinZoomy(_) => msg[3] = 0x0b,
            SetAuraBuiltin::WideZoomy(_) => msg[3] = 0x0c,
            _ => panic!("Mode not convertable to array"),
        }

        match mode {
            SetAuraBuiltin::Rainbow(settings) => {
                msg[7] = settings.speed as u8;
                msg[8] = settings.direction as u8;
            }
            SetAuraBuiltin::Breathe(settings) => {
                msg[3] = 0x01;
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
                msg[7] = settings.speed as u8;
                msg[10] = settings.colour2.0;
                msg[11] = settings.colour2.1;
                msg[12] = settings.colour2.2;
            }
            SetAuraBuiltin::Cycle(settings) | SetAuraBuiltin::Disco(settings) => {
                msg[7] = settings.speed as u8;
            }
            SetAuraBuiltin::Rain(settings)
            | SetAuraBuiltin::Highlight(settings)
            | SetAuraBuiltin::Laser(settings)
            | SetAuraBuiltin::Ripple(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
                msg[7] = settings.speed as u8;
            }
            SetAuraBuiltin::Stable(settings)
            | SetAuraBuiltin::Pulse(settings)
            | SetAuraBuiltin::ThinZoomy(settings)
            | SetAuraBuiltin::WideZoomy(settings) => {
                msg[4] = settings.colour.0;
                msg[5] = settings.colour.1;
                msg[6] = settings.colour.2;
            }
            _ => panic!("Mode not convertable to array"),
        }
        msg
    }
}

impl From<SetAuraBuiltin> for [[u8; LED_MSG_LEN]; 4] {
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

/// Container for the byte strings used in modes. Generally useful for settings
/// and other usecases.
#[derive(Deserialize, Serialize)]
pub struct BuiltInModeBytes {
    pub stable: [u8; LED_MSG_LEN],
    pub breathe: [u8; LED_MSG_LEN],
    pub cycle: [u8; LED_MSG_LEN],
    pub rainbow: [u8; LED_MSG_LEN],
    pub rain: [u8; LED_MSG_LEN],
    pub random: [u8; LED_MSG_LEN],
    pub highlight: [u8; LED_MSG_LEN],
    pub laser: [u8; LED_MSG_LEN],
    pub ripple: [u8; LED_MSG_LEN],
    pub pulse: [u8; LED_MSG_LEN],
    pub thinzoomy: [u8; LED_MSG_LEN],
    pub widezoomy: [u8; LED_MSG_LEN],
    pub multi_static: [[u8; LED_MSG_LEN]; 4],
}
impl BuiltInModeBytes {
    pub fn set_field_from(&mut self, bytes: &[u8]) {
        if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            let b = BuiltInModeByte::from(bytes[3]);
            match b {
                BuiltInModeByte::Single => self.stable.copy_from_slice(bytes),
                BuiltInModeByte::Breathing => self.breathe.copy_from_slice(bytes),
                BuiltInModeByte::Cycle => self.cycle.copy_from_slice(bytes),
                BuiltInModeByte::Rainbow => self.rainbow.copy_from_slice(bytes),
                BuiltInModeByte::Rain => self.rain.copy_from_slice(bytes),
                BuiltInModeByte::Random => self.random.copy_from_slice(bytes),
                BuiltInModeByte::Highlight => self.highlight.copy_from_slice(bytes),
                BuiltInModeByte::Laser => self.laser.copy_from_slice(bytes),
                BuiltInModeByte::Ripple => self.ripple.copy_from_slice(bytes),
                BuiltInModeByte::Pulse => self.pulse.copy_from_slice(bytes),
                BuiltInModeByte::ThinZoomy => self.thinzoomy.copy_from_slice(bytes),
                BuiltInModeByte::WideZoomy => self.widezoomy.copy_from_slice(bytes),
                _ => {}
            }
        }
    }

    pub fn get_field_from(&mut self, byte: u8) -> Option<&[u8]> {
        let bytes = match BuiltInModeByte::from(byte) {
            BuiltInModeByte::Single => &self.stable,
            BuiltInModeByte::Breathing => &self.breathe,
            BuiltInModeByte::Cycle => &self.cycle,
            BuiltInModeByte::Rainbow => &self.rainbow,
            BuiltInModeByte::Rain => &self.rain,
            BuiltInModeByte::Random => &self.random,
            BuiltInModeByte::Highlight => &self.highlight,
            BuiltInModeByte::Laser => &self.laser,
            BuiltInModeByte::Ripple => &self.ripple,
            BuiltInModeByte::Pulse => &self.pulse,
            BuiltInModeByte::ThinZoomy => &self.thinzoomy,
            BuiltInModeByte::WideZoomy => &self.widezoomy,
            _ => return None,
        };
        return Some(bytes);
    }
}
impl Default for BuiltInModeBytes {
    fn default() -> Self {
        BuiltInModeBytes {
            stable: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Stable(SingleColour::default())),
            breathe: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Breathe(TwoColourSpeed::default())),
            cycle: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Cycle(SingleSpeed::default())),
            rainbow: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Rainbow(
                SingleSpeedDirection::default(),
            )),
            rain: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Rain(SingleColourSpeed::default())),
            random: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Disco(SingleSpeed::default())),
            highlight: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Highlight(
                SingleColourSpeed::default(),
            )),
            laser: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Laser(SingleColourSpeed::default())),
            ripple: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Ripple(SingleColourSpeed::default())),
            pulse: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Pulse(SingleColour::default())),
            thinzoomy: <[u8; LED_MSG_LEN]>::from(
                SetAuraBuiltin::ThinZoomy(SingleColour::default()),
            ),
            widezoomy: <[u8; LED_MSG_LEN]>::from(
                SetAuraBuiltin::WideZoomy(SingleColour::default()),
            ),
            multi_static: <[[u8; LED_MSG_LEN]; 4]>::from(SetAuraBuiltin::MultiStatic(
                MultiColour::default(),
            )),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum BuiltInModeByte {
    Single = 0x00,
    Breathing = 0x01,
    Cycle = 0x02,
    Rainbow = 0x03,
    Rain = 0x04,
    Random = 0x05,
    Highlight = 0x06,
    Laser = 0x07,
    Ripple = 0x08,
    Pulse = 0x0a,
    ThinZoomy = 0x0b,
    WideZoomy = 0x0c,
    None,
}
impl Default for BuiltInModeByte {
    fn default() -> Self {
        BuiltInModeByte::Single
    }
}

impl From<u8> for BuiltInModeByte {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Single,
            0x01 => Self::Breathing,
            0x02 => Self::Cycle,
            0x03 => Self::Rainbow,
            0x04 => Self::Rain,
            0x05 => Self::Random,
            0x06 => Self::Highlight,
            0x07 => Self::Laser,
            0x08 => Self::Ripple,
            0x0a => Self::Pulse,
            0x0b => Self::ThinZoomy,
            0x0c => Self::WideZoomy,
            _ => Self::None,
        }
    }
}

impl From<&u8> for BuiltInModeByte {
    fn from(byte: &u8) -> Self {
        Self::from(*byte)
    }
}

impl From<BuiltInModeByte> for u8 {
    fn from(byte: BuiltInModeByte) -> Self {
        match byte {
            BuiltInModeByte::Single => 0x00,
            BuiltInModeByte::Breathing => 0x01,
            BuiltInModeByte::Cycle => 0x02,
            BuiltInModeByte::Rainbow => 0x03,
            BuiltInModeByte::Rain => 0x04,
            BuiltInModeByte::Random => 0x05,
            BuiltInModeByte::Highlight => 0x06,
            BuiltInModeByte::Laser => 0x07,
            BuiltInModeByte::Ripple => 0x08,
            BuiltInModeByte::Pulse => 0x0a,
            BuiltInModeByte::ThinZoomy => 0x0b,
            BuiltInModeByte::WideZoomy => 0x0c,
            BuiltInModeByte::None => 0xff,
        }
    }
}

#[derive(Clone)]
pub struct KeyColourArray([[u8; 64]; 10]);
impl KeyColourArray {
    pub fn new() -> Self {
        let mut set = [[0u8; 64]; 10];
        for (count, row) in set.iter_mut().enumerate() {
            row[0] = 0x5d; // Report ID
            row[1] = 0xbc; // Mode = custom??, 0xb3 is builtin
            row[2] = 0x00;
            row[3] = 0x01; // ??
            row[4] = 0x01; // ??, 4,5,6 are normally RGB for builtin mode colours
            row[5] = 0x01; // ??
            row[6] = (count as u8) << 4; // Key group
            row[7] = 0x10; // 0b00010000 addressing? flips for group a0
            if count == 9 {
                row[7] = 0x08; // 0b00001000
            }
            row[8] = 0x00;
        }
        KeyColourArray(set)
    }

    /// Initialise and clear the keyboard for custom effects
    pub fn get_init_msg() -> Vec<u8> {
        let mut init = vec![0u8; 64];
        init[0] = 0x5d; // Report ID
        init[1] = 0xbc; // Mode = custom??, 0xb3 is builtin
        init
    }

    pub fn set(&mut self, key: Key, r: u8, g: u8, b: u8) {
        let (rr, gg, bb) = self.key(key);
        *rr = r;
        *gg = g;
        *bb = b;
    }

    pub fn key(&mut self, key: Key) -> (&mut u8, &mut u8, &mut u8) {
        // Tuples are indexes in to array
        let (row, col) = match key {
            Key::VolUp => (0, 15),
            Key::VolDown => (0, 18),
            Key::MicMute => (0, 21),
            Key::ROG => (0, 24),
            //
            Key::Esc => (1, 24),
            Key::F1 => (1, 30),
            Key::F2 => (1, 33),
            Key::F3 => (1, 36),
            Key::F4 => (1, 39),
            Key::F5 => (1, 45),
            Key::F6 => (1, 48),
            Key::F7 => (1, 51),
            Key::F8 => (1, 54),
            //
            Key::F9 => (2, 12),
            Key::F10 => (2, 15),
            Key::F11 => (2, 18),
            Key::F12 => (2, 21),
            Key::Del => (2, 24),
            Key::Tilde => (2, 39),
            Key::N1 => (2, 42),
            Key::N2 => (2, 45),
            Key::N3 => (2, 48),
            Key::N4 => (2, 51),
            Key::N5 => (2, 54),
            //
            Key::N6 => (3, 9),
            Key::N7 => (3, 12),
            Key::N8 => (3, 15),
            Key::N9 => (3, 18),
            Key::N0 => (3, 21),
            Key::Hyphen => (3, 24),
            Key::Equals => (3, 27),
            Key::BkSpc1 => (3, 30),
            Key::BkSpc2 => (3, 33),
            Key::BkSpc3 => (3, 36),
            Key::Home => (3, 39),
            Key::Tab => (3, 54),
            //
            Key::Q => (4, 9),
            Key::W => (4, 12),
            Key::E => (4, 15),
            Key::R => (4, 18),
            Key::T => (4, 21),
            Key::Y => (4, 24),
            Key::U => (4, 27),
            Key::I => (4, 30),
            Key::O => (4, 33),
            Key::P => (4, 36),
            Key::LBracket => (4, 39),
            Key::RBracket => (4, 42),
            Key::BackSlash => (4, 45),
            Key::PgUp => (4, 54),
            //
            Key::Caps => (5, 21),
            Key::A => (5, 24),
            Key::S => (5, 27),
            Key::D => (5, 30),
            Key::F => (5, 33),
            Key::G => (5, 36),
            Key::H => (5, 39),
            Key::J => (5, 42),
            Key::K => (5, 45),
            Key::L => (5, 48),
            Key::SemiColon => (5, 51),
            Key::Quote => (5, 54),
            //
            Key::Ret1 => (6, 12),
            Key::Ret2 => (6, 15),
            Key::Ret3 => (6, 18),
            Key::PgDn => (6, 21),
            Key::LShift => (6, 36),
            Key::Z => (6, 42),
            Key::X => (6, 45),
            Key::C => (6, 48),
            Key::V => (6, 51),
            Key::B => (6, 54),
            //
            Key::N => (7, 9),
            Key::M => (7, 12),
            Key::Comma => (7, 15),
            Key::Period => (7, 18),
            Key::FwdSlash => (7, 21),
            Key::Rshift1 => (7, 27),
            Key::Rshift2 => (7, 30),
            Key::Rshift3 => (7, 33),
            Key::End => (7, 36),
            Key::LCtrl => (7, 51),
            Key::LFn => (7, 54),
            //
            Key::Meta => (8, 9),
            Key::LAlt => (8, 12),
            Key::Space1 => (8, 15),
            Key::Space2 => (8, 18),
            Key::Space3 => (8, 21),
            Key::Space4 => (8, 24),
            Key::RAlt => (8, 30),
            Key::PrtSc => (8, 33),
            Key::RCtrl => (8, 36),
            Key::Up => (8, 42),
            Key::RFn => (8, 51),
            //
            Key::Left => (9, 54),
            //
            Key::Down => (10, 9),
            Key::Right => (10, 12),
        };
        // LOLOLOLOLOLOLOL! Look it's safe okay
        unsafe {
            (
                &mut *(&mut self.0[row][col] as *mut u8),
                &mut *(&mut self.0[row][col + 1] as *mut u8),
                &mut *(&mut self.0[row][col + 2] as *mut u8),
            )
        }
    }

    pub fn get(&self) -> &[[u8; 64]; 10] {
        &self.0
    }
}

pub enum Key {
    VolUp,
    VolDown,
    MicMute,
    ROG,
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Del,
    Tilde,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Hyphen,
    Equals,
    BkSpc1,
    BkSpc2,
    BkSpc3,
    Home,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LBracket,
    RBracket,
    BackSlash,
    PgUp,
    Caps,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    SemiColon,
    Quote,
    Ret1,
    Ret2,
    Ret3,
    PgDn,
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    FwdSlash,
    Rshift1,
    Rshift2,
    Rshift3,
    End,
    LCtrl,
    LFn,
    Meta,
    LAlt,
    Space1,
    Space2,
    Space3,
    Space4,
    RAlt,
    PrtSc,
    RCtrl,
    Up,
    Down,
    Left,
    Right,
    RFn,
}
