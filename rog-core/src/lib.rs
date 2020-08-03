#![deny(unused_must_use)]
/// Configuration loading, saving
mod config;
///
mod ctrl_anime;
///
mod ctrl_charge;
///
pub mod ctrl_fan_cpu;
///
mod ctrl_leds;
/// Start the daemon loop
pub mod daemon;
///
mod dbus;
/// Laptop matching to determine capabilities
mod laptops;

mod error;
