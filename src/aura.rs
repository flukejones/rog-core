use crate::core::LED_MSG_LEN;
use gumdrop::Options;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum AuraError {
    ParseColour,
    ParseSpeed,
    ParseDirection,
    ParseBrightness,
}

impl Debug for AuraError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self.description(), f)
    }
}

impl Display for AuraError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self.description(), f)
    }
}

impl Error for AuraError {
    fn description(&self) -> &str {
        match self {
            AuraError::ParseColour => "could not parse colour",
            AuraError::ParseSpeed => "could not parse speed",
            AuraError::ParseDirection => "could not parse direction",
            AuraError::ParseBrightness => "could not parse brightness",
        }
    }
}

#[derive(Default, Debug, PartialEq, Options)]
pub struct Colour {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "red: eg, 255")]
    r: u8,
    #[options(help = "green: eg, 123")]
    g: u8,
    #[options(help = "blue: eg, 166")]
    b: u8,
}

impl FromStr for Colour {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 6 {
            return Err(AuraError::ParseColour);
        }
        let r = u8::from_str_radix(&s[0..2], 16).or(Err(AuraError::ParseColour))?;
        let g = u8::from_str_radix(&s[2..4], 16).or(Err(AuraError::ParseColour))?;
        let b = u8::from_str_radix(&s[4..6], 16).or(Err(AuraError::ParseColour))?;
        Ok(Colour {
            help: false,
            r,
            g,
            b,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Speed {
    Slow = 0xe1,
    Medium = 0xeb,
    Fast = 0xf5,
}
impl Default for Speed {
    fn default() -> Self {
        Speed::Slow
    }
}
impl FromStr for Speed {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "low" => Ok(Speed::Slow),
            "med" => Ok(Speed::Medium),
            "high" => Ok(Speed::Fast),
            _ => Err(AuraError::ParseSpeed),
        }
    }
}

/// Used for Rainbow mode.
///
/// Enum corresponds to the required integer value
#[derive(Debug, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
impl FromStr for Direction {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "right" => Ok(Direction::Right),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            _ => Err(AuraError::ParseDirection),
        }
    }
}

/// Byte value for setting the built-in mode.
///
/// Enum corresponds to the required integer value
#[derive(Debug, Options)]
pub enum SetAuraBuiltin {
    #[options(help = "set a single static colour")]
    Stable(Colour), // colour1
    #[options(help = "pulse between one or two colours")]
    Breathe(Breathe), // colour1, colour2, speed
    #[options(help = "cycle through all colours")]
    Cycle(SingleSpeed), // speed
    // Rainbow,      // speed, direction
    #[options(help = "random pattern mimicing raindrops")]
    Rain(SingleColourSpeed), // colour1, speed
    #[options(help = "random pattern of three preset colours")]
    Random(SingleSpeed), // speed
    #[options(help = "pressed keys are highlighted to fade")]
    Highlight(SingleColourSpeed), // colour1, speed
    #[options(help = "pressed keys generate horizontal laser")]
    Laser(SingleColourSpeed), // colour1, speed
    #[options(help = "pressed keys ripple outwards like a splash")]
    Ripple(SingleColourSpeed), // colour1, speed
    // Off,          // none
    #[options(help = "set a rapid pulse")]
    Pulse(Colour), // colour1
    #[options(help = "set a vertical line racing from left")]
    LineRace(Colour), // colour1
    #[options(help = "set a wide vertical line racing from left")]
    WideLineRace(Colour), // colour1
}

impl Default for SetAuraBuiltin {
    fn default() -> Self {
        SetAuraBuiltin::Stable(Colour::default())
    }
}

#[derive(Debug, PartialEq, Options)]
pub struct Breathe {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "set the first colour, must be hex string e.g, ff00ff")]
    colour1: Colour,
    #[options(help = "set the second colour, must be hex string e.g, ff00ff")]
    colour2: Colour,
    #[options(help = "set the speed")]
    speed: Speed,
}

#[derive(Debug, PartialEq, Options)]
pub struct SingleSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "set the speed")]
    speed: Speed,
}

#[derive(Debug, PartialEq, Options)]
pub struct SingleColourSpeed {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "set the colour, must be hex string e.g, ff00ff")]
    colour: Colour,
    #[options(help = "set the speed")]
    speed: Speed,
}

/// Packet Data:
///
/// ```
/// | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11| 12| 13|
/// |---|---|---|---|---|---|---|---|---|---|---|---|---|
/// |5d |b3 |03 |00 |ff |00 |00 |00 |00 |00 |00 |ff |00 |
/// ```
///
/// Bytes 1 and 2 should always be 5d, b3
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
/// - 09 = off
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
pub struct ModeMessage(pub [u8; LED_MSG_LEN]);

impl From<SetAuraBuiltin> for ModeMessage {
    fn from(mode: SetAuraBuiltin) -> Self {
        let mut msg = [0u8; LED_MSG_LEN];
        msg[0] = 0x5d;
        msg[1] = 0xb3;
        match mode {
            SetAuraBuiltin::Stable(_) => {
                msg[3] = 0x00;
            }
            SetAuraBuiltin::Cycle(_) => {
                msg[3] = 0x02;
            }
            SetAuraBuiltin::Rain(_) => {
                msg[3] = 0x04;
            }
            SetAuraBuiltin::Random(_) => {
                msg[3] = 0x05;
            }
            SetAuraBuiltin::Highlight(_) => {
                msg[3] = 0x06;
            }
            SetAuraBuiltin::Laser(_) => {
                msg[3] = 0x07;
            }
            SetAuraBuiltin::Ripple(_) => {
                msg[3] = 0x08;
            }
            SetAuraBuiltin::Pulse(_) => {
                msg[3] = 0x0a;
            }
            SetAuraBuiltin::LineRace(_) => {
                msg[3] = 0x0b;
            }
            SetAuraBuiltin::WideLineRace(_) => {
                msg[3] = 0x0c;
            }
            _ => {}
        }
        match mode {
            SetAuraBuiltin::Breathe(settings) => {
                msg[3] = 0x01;
                msg[4] = settings.colour1.r;
                msg[5] = settings.colour1.g;
                msg[6] = settings.colour1.b;
                msg[7] = settings.speed as u8;
                msg[10] = settings.colour2.r;
                msg[11] = settings.colour2.g;
                msg[12] = settings.colour2.b;
                return ModeMessage(msg);
            }
            SetAuraBuiltin::Cycle(settings) | SetAuraBuiltin::Random(settings) => {
                msg[7] = settings.speed as u8;
                return ModeMessage(msg);
            }
            SetAuraBuiltin::Rain(settings)
            | SetAuraBuiltin::Highlight(settings)
            | SetAuraBuiltin::Laser(settings)
            | SetAuraBuiltin::Ripple(settings) => {
                msg[4] = settings.colour.r;
                msg[5] = settings.colour.g;
                msg[6] = settings.colour.b;
                msg[7] = settings.speed as u8;
                return ModeMessage(msg);
            }
            SetAuraBuiltin::Stable(settings)
            | SetAuraBuiltin::Pulse(settings)
            | SetAuraBuiltin::LineRace(settings)
            | SetAuraBuiltin::WideLineRace(settings) => {
                msg[4] = settings.r;
                msg[5] = settings.g;
                msg[6] = settings.b;
                return ModeMessage(msg);
            }
        }
    }
}
