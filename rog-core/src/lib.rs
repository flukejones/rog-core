#![deny(unused_must_use)]
///
mod animatrix_control;
/// Configuration loading, saving
mod config;
/// Start the daemon loop
pub mod daemon;
/// Laptop matching to determine capabilities
mod laptops;
///
mod led_control;
///
mod rog_dbus;
/// The core module which allows writing to LEDs or polling the
/// laptop keyboard attached devices
pub mod rogcore;

mod error;
