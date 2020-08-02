// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

use crate::{config::Config, error::RogError};
use log::{error, info, warn};
use rog_client::{
    aura_brightness_bytes, aura_modes::AuraModes, fancy::KeyColourArray, LED_MSG_LEN,
};
use std::fs::OpenOptions;
use std::io::Write;

/// UNSAFE: Must live as long as RogCore
///
/// Because we're holding a pointer to something that *may* go out of scope while the
/// pointer is held. We're relying on access to struct to be behind a Mutex, and for behaviour
/// that may cause invalididated pointer to cause the program to panic rather than continue.
pub struct LedWriter {
    dev_node: String,
    supported_modes: Vec<u8>,
    flip_effect_write: bool,
}

impl LedWriter {
    #[inline]
    pub fn new(dev_node: String, supported_modes: Vec<u8>) -> Self {
        LedWriter {
            dev_node,
            supported_modes,
            flip_effect_write: false,
        }
    }

    pub async fn do_command(
        &mut self,
        mode: AuraModes,
        config: &mut Config,
    ) -> Result<(), RogError> {
        self.set_and_save(mode, config).await
    }

    /// Should only be used if the bytes you are writing are verified correct
    #[inline]
    async fn write_bytes(&self, message: &[u8]) -> Result<(), RogError> {
        if let Ok(mut file) = OpenOptions::new().write(true).open(&self.dev_node) {
            file.write_all(message).unwrap();
            return Ok(());
        }
        Err(RogError::NotSupported)
    }

    /// Write an effect block
    ///
    /// `aura_effect_init` must be called any effect routine, and called only once.
    #[inline]
    async fn write_effect(&mut self, effect: &[Vec<u8>]) -> Result<(), RogError> {
        if self.flip_effect_write {
            for row in effect.iter().rev() {
                self.write_bytes(row).await?;
            }
        } else {
            for row in effect.iter() {
                self.write_bytes(row).await?;
            }
        }
        self.flip_effect_write = !self.flip_effect_write;
        Ok(())
    }

    /// Used to set a builtin mode and save the settings for it
    ///
    /// This needs to be universal so that settings applied by dbus stick
    #[inline]
    async fn set_and_save(&mut self, mode: AuraModes, config: &mut Config) -> Result<(), RogError> {
        match mode {
            AuraModes::LedBrightness(n) => {
                let bytes: [u8; LED_MSG_LEN] = (&mode).into();
                self.write_bytes(&bytes).await?;
                config.brightness = n;
                config.write();
                info!("LED brightness set to {:#?}", n);
            }
            _ => {
                let mode_num: u8 = u8::from(&mode);
                self.write_mode(&mode).await?;
                config.current_mode = mode_num;
                config.set_mode_data(mode);
                config.write();
            }
        }
        Ok(())
    }

    #[inline]
    async fn write_mode(&mut self, mode: &AuraModes) -> Result<(), RogError> {
        match mode {
            AuraModes::RGB(v) => {
                if v.is_empty() || v[0].is_empty() {
                    let bytes = KeyColourArray::get_init_msg();
                    self.write_bytes(&bytes).await?;
                } else {
                    self.write_effect(v).await?;
                }
            }
            _ => {
                let mode_num: u8 = u8::from(mode);
                match mode {
                    AuraModes::MultiStatic(_) => {
                        if self.supported_modes.contains(&mode_num) {
                            let bytes: [[u8; LED_MSG_LEN]; 4] = mode.into();
                            for array in bytes.iter() {
                                self.write_bytes(array).await?;
                            }
                        }
                    }
                    _ => {
                        if self.supported_modes.contains(&mode_num) {
                            let bytes: [u8; LED_MSG_LEN] = mode.into();
                            self.write_bytes(&bytes).await?;
                        }
                    }
                }
                self.write_bytes(&LED_SET).await?;
                // Changes won't persist unless apply is set
                self.write_bytes(&LED_APPLY).await?;
            }
        }
        Ok(())
    }

    #[inline]
    pub async fn reload_last_builtin(&mut self, config: &mut Config) -> Result<(), RogError> {
        // set current mode (if any)
        if self.supported_modes.len() > 1 {
            if self.supported_modes.contains(&config.current_mode) {
                let mode = config
                    .get_led_mode_data(config.current_mode)
                    .ok_or(RogError::NotSupported)?
                    .to_owned();
                self.write_mode(&mode).await?;
                info!("Reloaded last used mode");
            } else {
                warn!(
                    "An unsupported mode was set: {}, reset to first mode available",
                    <&str>::from(&<AuraModes>::from(config.current_mode))
                );
                for (idx, mode) in config.builtin_modes.iter_mut().enumerate() {
                    if !self.supported_modes.contains(&mode.into()) {
                        config.builtin_modes.remove(idx);
                        config.write();
                        break;
                    }
                }
                config.current_mode = self.supported_modes[0];
                // TODO: do a recursive call with a boxed dyn future later
                let mode = config
                    .get_led_mode_data(config.current_mode)
                    .ok_or(RogError::NotSupported)?
                    .to_owned();
                self.write_mode(&mode).await?;
                info!("Reloaded last used mode");
            }
        }

        // Reload brightness
        let bright = config.brightness;
        let bytes = aura_brightness_bytes(bright);
        self.write_bytes(&bytes).await?;
        info!("Reloaded last used brightness");
        Ok(())
    }
}
