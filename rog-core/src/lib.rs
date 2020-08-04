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

use async_trait::async_trait;
use config::Config;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio::task::JoinHandle;

use ::dbus::{nonblock::SyncConnection, tree::Signal};

#[async_trait]
pub trait Controller {
    type A;

    async fn reload_from_config(&mut self, config: &mut Config) -> Result<(), Box<dyn Error>>;

    /// Spawn an infinitely running task (usually) which checks a Receiver for input,
    /// and may send a signal over dbus
    fn spawn_task(
        self,
        config: Arc<Mutex<Config>>,
        recv: Receiver<Self::A>,
        connection: Option<Arc<SyncConnection>>,
        signal: Option<Arc<Signal<()>>>,
    ) -> JoinHandle<()>;
}
