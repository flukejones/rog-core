use super::cli_options::*;
use super::LED_MSG_LEN;
use serde_derive::{Deserialize, Serialize};

/// Container for the byte strings used in modes. Generally useful for settings
/// and other usecases.
#[derive(Deserialize, Serialize)]
pub struct BuiltInModeBytes {
    pub stable: [u8; LED_MSG_LEN],
    pub breathe: [u8; LED_MSG_LEN],
    pub strobe: [u8; LED_MSG_LEN],
    pub rainbow: [u8; LED_MSG_LEN],
    pub star: [u8; LED_MSG_LEN],
    pub rain: [u8; LED_MSG_LEN],
    pub highlight: [u8; LED_MSG_LEN],
    pub laser: [u8; LED_MSG_LEN],
    pub ripple: [u8; LED_MSG_LEN],
    pub pulse: [u8; LED_MSG_LEN],
    pub comet: [u8; LED_MSG_LEN],
    pub flash: [u8; LED_MSG_LEN],
    pub multi_static: [[u8; LED_MSG_LEN]; 4],
}
impl BuiltInModeBytes {
    #[inline]
    pub fn set_field_from(&mut self, bytes: &[u8]) {
        if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            let b = BuiltInModeByte::from(bytes[3]);
            match b {
                BuiltInModeByte::Single => self.stable.copy_from_slice(bytes),
                BuiltInModeByte::Breathing => self.breathe.copy_from_slice(bytes),
                BuiltInModeByte::Strobe => self.strobe.copy_from_slice(bytes),
                BuiltInModeByte::Rainbow => self.rainbow.copy_from_slice(bytes),
                BuiltInModeByte::Star => self.star.copy_from_slice(bytes),
                BuiltInModeByte::Rain => self.rain.copy_from_slice(bytes),
                BuiltInModeByte::Highlight => self.highlight.copy_from_slice(bytes),
                BuiltInModeByte::Laser => self.laser.copy_from_slice(bytes),
                BuiltInModeByte::Ripple => self.ripple.copy_from_slice(bytes),
                BuiltInModeByte::Pulse => self.pulse.copy_from_slice(bytes),
                BuiltInModeByte::Comet => self.comet.copy_from_slice(bytes),
                BuiltInModeByte::Flash => self.flash.copy_from_slice(bytes),
                _ => {}
            }
        }
    }

    #[inline]
    pub fn get_field_from(&self, byte: u8) -> Option<&[u8]> {
        let bytes = match BuiltInModeByte::from(byte) {
            BuiltInModeByte::Single => &self.stable,
            BuiltInModeByte::Breathing => &self.breathe,
            BuiltInModeByte::Strobe => &self.strobe,
            BuiltInModeByte::Rainbow => &self.rainbow,
            BuiltInModeByte::Star => &self.star,
            BuiltInModeByte::Rain => &self.rain,
            BuiltInModeByte::Highlight => &self.highlight,
            BuiltInModeByte::Laser => &self.laser,
            BuiltInModeByte::Ripple => &self.ripple,
            BuiltInModeByte::Pulse => &self.pulse,
            BuiltInModeByte::Comet => &self.comet,
            BuiltInModeByte::Flash => &self.flash,
            _ => return None,
        };
        Some(bytes)
    }
}
impl Default for BuiltInModeBytes {
    fn default() -> Self {
        BuiltInModeBytes {
            stable: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Stable(SingleColour::default())),
            breathe: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Breathe(TwoColourSpeed::default())),
            strobe: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Strobe(SingleSpeed::default())),
            rainbow: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Rainbow(
                SingleSpeedDirection::default(),
            )),
            star: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Star(TwoColourSpeed::default())),
            rain: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Rain(SingleSpeed::default())),
            highlight: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Highlight(
                SingleColourSpeed::default(),
            )),
            laser: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Laser(SingleColourSpeed::default())),
            ripple: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Ripple(SingleColourSpeed::default())),
            pulse: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Pulse(SingleColour::default())),
            comet: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Comet(SingleColour::default())),
            flash: <[u8; LED_MSG_LEN]>::from(SetAuraBuiltin::Flash(SingleColour::default())),
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
    Strobe = 0x02,
    Rainbow = 0x03,
    Star = 0x04,
    Rain = 0x05,
    Highlight = 0x06,
    Laser = 0x07,
    Ripple = 0x08,
    Pulse = 0x0a,
    Comet = 0x0b,
    Flash = 0x0c,
    MultiStatic,
    None,
}
impl Default for BuiltInModeByte {
    #[inline]
    fn default() -> Self {
        BuiltInModeByte::Single
    }
}

impl From<u8> for BuiltInModeByte {
    #[inline]
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::Single,
            0x01 => Self::Breathing,
            0x02 => Self::Strobe,
            0x03 => Self::Rainbow,
            0x04 => Self::Star,
            0x05 => Self::Rain,
            0x06 => Self::Highlight,
            0x07 => Self::Laser,
            0x08 => Self::Ripple,
            0x0a => Self::Pulse,
            0x0b => Self::Comet,
            0x0c => Self::Flash,
            _ => Self::None,
        }
    }
}

impl From<&u8> for BuiltInModeByte {
    #[inline]
    fn from(byte: &u8) -> Self {
        Self::from(*byte)
    }
}

impl From<BuiltInModeByte> for u8 {
    #[inline]
    fn from(byte: BuiltInModeByte) -> Self {
        match byte {
            BuiltInModeByte::Single => 0x00,
            BuiltInModeByte::Breathing => 0x01,
            BuiltInModeByte::Strobe => 0x02,
            BuiltInModeByte::Rainbow => 0x03,
            BuiltInModeByte::Star => 0x04,
            BuiltInModeByte::Rain => 0x05,
            BuiltInModeByte::Highlight => 0x06,
            BuiltInModeByte::Laser => 0x07,
            BuiltInModeByte::Ripple => 0x08,
            BuiltInModeByte::Pulse => 0x0a,
            BuiltInModeByte::Comet => 0x0b,
            BuiltInModeByte::Flash => 0x0c,
            BuiltInModeByte::MultiStatic => 0x00,
            BuiltInModeByte::None => 0xff,
        }
    }
}
