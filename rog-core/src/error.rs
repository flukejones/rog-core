use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RogError {
    #[error("unable to parse string to fan mode")]
    ParseFanLevel,
    #[error("mode not supported")]
    NotSupported,
}
