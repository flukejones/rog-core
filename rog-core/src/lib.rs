/// Configuration loading, saving
mod config;
/// The core module which allows writing to LEDs or polling the
/// laptop keyboard attached devices
mod core;
/// Start the daemon loop
pub mod daemon;
/// Laptop matching to determine capabilities
mod laptops;
///
mod led_control;
/// A virtual "consumer device" to help emit the correct key codes
mod virt_device;
