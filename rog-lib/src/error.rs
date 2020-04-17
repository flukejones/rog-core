use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

#[derive(PartialEq)]
pub enum AuraError {
    ParseColour,
    ParseSpeed,
    ParseDirection,
    ParseBrightness,
    PollKeyboard,
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
            AuraError::PollKeyboard => "failed to poll keyboard",
        }
    }
}
