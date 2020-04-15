use crate::core::LED_MSG_LEN;
use gumdrop::Options;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum AuraError {
    ParseMode,
    ParseColour,
    ParseSpeed,
    ParseDirection,
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
            AuraError::ParseMode => "incorrect mode specified",
            AuraError::ParseColour => "could not parse colour",
            AuraError::ParseSpeed => "could not parse speed",
            AuraError::ParseDirection => "could not parse direction",
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Colour(u8, u8, u8);

impl FromStr for Colour {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 6 {
            return Err(AuraError::ParseColour);
        }
        let r = u8::from_str_radix(&s[0..2], 16).or(Err(AuraError::ParseColour))?;
        let g = u8::from_str_radix(&s[2..4], 16).or(Err(AuraError::ParseColour))?;
        let b = u8::from_str_radix(&s[4..6], 16).or(Err(AuraError::ParseColour))?;
        Ok(Colour(r, g, b))
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
            "slow" => Ok(Speed::Slow),
            "medium" => Ok(Speed::Medium),
            "fast" => Ok(Speed::Fast),
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
#[derive(Debug, PartialEq)]
pub enum ModeByte {
    Stable,       // colour1
    Breathe,      // colour1, colour2, speed
    Cycle,        // speed
    Rainbow,      // speed, direction
    Rain,         // colour1, speed
    Random,       // speed
    Highlight,    // colour1, speed
    Laser,        // colour1, speed
    Ripple,       // colour1, speed
    Off,          // none
    Pulse,        // colour1
    LineRace,     // colour1
    WideLineRace, // colour1
}
impl Default for ModeByte {
    fn default() -> Self {
        ModeByte::Stable
    }
}
impl FromStr for ModeByte {
    type Err = AuraError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stable" => Ok(ModeByte::Stable),
            "breathe" => Ok(ModeByte::Breathe),
            "cycle" => Ok(ModeByte::Cycle),
            "rainbow" => Ok(ModeByte::Rainbow),
            "rain" => Ok(ModeByte::Rain),
            "random" => Ok(ModeByte::Random),
            "highlight" => Ok(ModeByte::Highlight),
            "laser" => Ok(ModeByte::Laser),
            "ripple" => Ok(ModeByte::Ripple),
            "off" => Ok(ModeByte::Off),
            "pulse" => Ok(ModeByte::Pulse),
            "linerace" => Ok(ModeByte::LineRace),
            "widelinerace" => Ok(ModeByte::WideLineRace),
            _ => Err(AuraError::ParseMode),
        }
    }
}

#[derive(Debug, PartialEq, Options)]
pub struct BuiltInMode {
    #[options(help = "selects from the built-in keyboard modes")]
    mode: ModeByte,
    #[options(help = "set the first colour if a mode uses it, must be in hex: ffffff")]
    colour1: Colour,
    #[options(help = "set the second colour if a mode uses it, must be in hex: ffffff")]
    colour2: Colour,
    #[options(help = "set the speed of an effect if a mode uses it")]
    speed: Speed,
    #[options(help = "set the direction of an effect if a mode uses it")]
    direction: Direction,
}

impl Default for BuiltInMode {
    fn default() -> BuiltInMode {
        BuiltInMode {
            mode: ModeByte::Stable,
            colour1: Colour(0xff, 0x00, 0x00),
            colour2: Colour(0x00, 0x00, 0x00),
            speed: Speed::Slow,
            direction: Direction::Right,
        }
    }
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

impl From<BuiltInMode> for ModeMessage {
    fn from(mode: BuiltInMode) -> Self {
        let mut msg = [0u8; LED_MSG_LEN];
        msg[0] = 0x5d;
        msg[1] = 0xb3;
        msg[3] = mode.mode as u8;
        msg[4] = mode.colour1.0;
        msg[5] = mode.colour1.1;
        msg[6] = mode.colour1.2;
        msg[7] = mode.speed as u8;
        msg[8] = mode.direction as u8;
        msg[10] = mode.colour2.0;
        msg[11] = mode.colour2.1;
        msg[12] = mode.colour2.2;

        ModeMessage(msg)
    }
}
