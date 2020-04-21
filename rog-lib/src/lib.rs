mod aura;
/// Contains mostly only what is required for parsing CLI options
pub mod cli_options;
mod config;
/// The core module which allows writing to LEDs or polling the
/// laptop keyboard attached devices
pub mod core;
mod error;
pub mod laptops;
mod virt_device;
