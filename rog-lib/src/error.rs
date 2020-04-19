use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuraError {
    #[error("unable to parse string to colour")]
    ParseColour,
    #[error("unable to parse string to speed")]
    ParseSpeed,
    #[error("unable to parse string to direction")]
    ParseDirection,
    #[error("unable to parse string to brightness")]
    ParseBrightness,
    #[error("could not poll the keyboard for input")]
    PollKeyboard,
    #[error("mode not supported")]
    NotSupported,
    #[error("USB error")]
    UsbError(rusb::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("external command failed")]
    CommandFailed,
}
