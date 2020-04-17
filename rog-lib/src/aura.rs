use crate::cli_options::*;
use crate::core::LED_MSG_LEN;
use serde_derive::{Deserialize, Serialize};

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
            SetAuraBuiltin::Random(_) => msg[3] = 0x05,
            SetAuraBuiltin::Highlight(_) => msg[3] = 0x06,
            SetAuraBuiltin::Laser(_) => msg[3] = 0x07,
            SetAuraBuiltin::Ripple(_) => msg[3] = 0x08,
            SetAuraBuiltin::Pulse(_) => msg[3] = 0x0a,
            SetAuraBuiltin::ThinZoomy(_) => msg[3] = 0x0b,
            SetAuraBuiltin::WideZoomy(_) => msg[3] = 0x0c,
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
            SetAuraBuiltin::Cycle(settings) | SetAuraBuiltin::Random(settings) => {
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
}
impl BuiltInModeBytes {
    pub fn set_field_from(&mut self, bytes: &[u8]) {
        if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            let b = BuiltInModeByte::from(bytes[3]);
            match b {
                BuiltInModeByte::Stable => self.stable.copy_from_slice(bytes),
                BuiltInModeByte::Breathe => self.breathe.copy_from_slice(bytes),
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
            BuiltInModeByte::Stable => &self.stable,
            BuiltInModeByte::Breathe => &self.breathe,
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
            random: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Random(SingleSpeed::default())),
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
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum BuiltInModeByte {
    Stable = 0x00,
    Breathe = 0x01,
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
        BuiltInModeByte::Stable
    }
}
impl From<u8> for BuiltInModeByte {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Stable,
            0x01 => Self::Breathe,
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
