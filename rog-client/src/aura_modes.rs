use crate::cli_options::*;
use serde_derive::{Deserialize, Serialize};

pub const SINGLE: u8 = 0x00;
pub const BREATHING: u8 = 0x01;
pub const STROBE: u8 = 0x02;
pub const RAINBOW: u8 = 0x03;
pub const STAR: u8 = 0x04;
pub const RAIN: u8 = 0x05;
pub const HIGHLIGHT: u8 = 0x06;
pub const LASER: u8 = 0x07;
pub const RIPPLE: u8 = 0x08;
pub const PULSE: u8 = 0x0a;
pub const COMET: u8 = 0x0b;
pub const FLASH: u8 = 0x0c;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuraModes {
    Stable(SingleColour),
    Breathe(TwoColourSpeed),
    Strobe(SingleSpeed),
    Rainbow(SingleSpeedDirection),
    Star(TwoColourSpeed),
    Rain(SingleSpeed),
    Highlight(SingleColourSpeed),
    Laser(SingleColourSpeed),
    Ripple(SingleColourSpeed),
    Pulse(SingleColour),
    Comet(SingleColour),
    Flash(SingleColour),
    MultiStatic(MultiColour),
    LedBrightness(u8),
    // TODO: use a serializable structure for this (KeyColourArray)
    RGB(Vec<Vec<u8>>),
}

impl From<SetAuraBuiltin> for AuraModes {
    fn from(mode: SetAuraBuiltin) -> Self {
        match mode {
            SetAuraBuiltin::Stable(x) => AuraModes::Stable(x),
            SetAuraBuiltin::Breathe(x) => AuraModes::Breathe(x),
            SetAuraBuiltin::Strobe(x) => AuraModes::Strobe(x),
            SetAuraBuiltin::Rainbow(x) => AuraModes::Rainbow(x),
            SetAuraBuiltin::Star(x) => AuraModes::Star(x),
            SetAuraBuiltin::Rain(x) => AuraModes::Rain(x),
            SetAuraBuiltin::Highlight(x) => AuraModes::Highlight(x),
            SetAuraBuiltin::Laser(x) => AuraModes::Laser(x),
            SetAuraBuiltin::Ripple(x) => AuraModes::Ripple(x),
            SetAuraBuiltin::Pulse(x) => AuraModes::Pulse(x),
            SetAuraBuiltin::Comet(x) => AuraModes::Comet(x),
            SetAuraBuiltin::Flash(x) => AuraModes::Flash(x),
            SetAuraBuiltin::MultiStatic(x) => AuraModes::MultiStatic(x),
        }
    }
}

impl From<AuraModes> for u8 {
    fn from(mode: AuraModes) -> Self {
        u8::from(&mode)
    }
}

impl From<&mut AuraModes> for u8 {
    fn from(mode: &mut AuraModes) -> Self {
        u8::from(&*mode)
    }
}

impl From<&AuraModes> for u8 {
    fn from(mode: &AuraModes) -> Self {
        match mode {
            AuraModes::Stable(_) => 0x00,
            AuraModes::Breathe(_) => 0x01,
            AuraModes::Strobe(_) => 0x02,
            AuraModes::Rainbow(_) => 0x03,
            AuraModes::Star(_) => 0x04,
            AuraModes::Rain(_) => 0x05,
            AuraModes::Highlight(_) => 0x06,
            AuraModes::Laser(_) => 0x07,
            AuraModes::Ripple(_) => 0x08,
            AuraModes::Pulse(_) => 0x0a,
            AuraModes::Comet(_) => 0x0b,
            AuraModes::Flash(_) => 0x0c,
            AuraModes::MultiStatic(_) => 0x0d,
            _ => panic!("Invalid mode"),
        }
    }
}

impl From<u8> for AuraModes {
    fn from(byte: u8) -> Self {
        match byte {
            SINGLE => AuraModes::Stable(SingleColour::default()),
            BREATHING => AuraModes::Breathe(TwoColourSpeed::default()),
            STROBE => AuraModes::Strobe(SingleSpeed::default()),
            RAINBOW => AuraModes::Rainbow(SingleSpeedDirection::default()),
            STAR => AuraModes::Star(TwoColourSpeed::default()),
            RAIN => AuraModes::Rain(SingleSpeed::default()),
            HIGHLIGHT => AuraModes::Highlight(SingleColourSpeed::default()),
            LASER => AuraModes::Laser(SingleColourSpeed::default()),
            RIPPLE => AuraModes::Ripple(SingleColourSpeed::default()),
            PULSE => AuraModes::Pulse(SingleColour::default()),
            COMET => AuraModes::Comet(SingleColour::default()),
            FLASH => AuraModes::Flash(SingleColour::default()),
            _ => panic!("Invalid mode byte"),
        }
    }
}
