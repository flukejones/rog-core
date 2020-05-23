#![deny(unused_must_use)]
/// Configuration loading, saving
mod config;
/// Start the daemon loop
pub mod daemon;
/// Laptop matching to determine capabilities
mod laptops;
///
mod led_control;
/// The core module which allows writing to LEDs or polling the
/// laptop keyboard attached devices
pub mod rogcore;
/// A virtual "consumer device" to help emit the correct key codes
mod virt_device;

mod error;
