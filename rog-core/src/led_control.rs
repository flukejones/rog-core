static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &str = "]ASUS Tech.Inc."; // ] == 0x5d
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0, 0x08];
static LED_INIT4: &str = "^ASUS Tech.Inc."; // ^ == 0x5e
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0, 0x08];

// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

use crate::config::Config;
use log::{error, info, warn};
use rog_aura::{aura_brightness_bytes, error::AuraError, BuiltInModeByte};
use rusb::DeviceHandle;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::time::Duration;

pub enum AuraCommand {
    BrightInc,
    BrightDec,
    BuiltinNext,
    BuiltinPrev,
    WriteBytes(Vec<u8>),
    WriteEffect(Vec<Vec<u8>>),
    ReloadLast,
    WriteMultizone(Vec<Vec<u8>>),
}

/// UNSAFE: Must live as long as RogCore
///
/// Because we're holding a pointer to something that *may* go out of scope while the
/// pointer is held. We're relying on access to struct to be behind a Mutex, and for behaviour
/// that may cause invalididated pointer to cause the program to panic rather than continue.
pub struct LedWriter<'d, C: 'd>
where
    C: rusb::UsbContext,
{
    handle: NonNull<DeviceHandle<C>>,
    bright_min_max: (u8, u8),
    supported_modes: Vec<BuiltInModeByte>,
    led_endpoint: u8,
    initialised: bool,
    flip_effect_write: bool,
    _phantom: PhantomData<&'d DeviceHandle<C>>,
}

/// UNSAFE
unsafe impl<'d, C> Send for LedWriter<'d, C> where C: rusb::UsbContext {}
unsafe impl<'d, C> Sync for LedWriter<'d, C> where C: rusb::UsbContext {}

impl<'d, C> LedWriter<'d, C>
where
    C: rusb::UsbContext,
{
    #[inline]
    pub fn new(
        device_handle: NonNull<DeviceHandle<C>>,
        led_endpoint: u8,
        bright_min_max: (u8, u8),
        supported_modes: Vec<BuiltInModeByte>,
    ) -> Self {
        LedWriter {
            handle: device_handle,
            led_endpoint,
            bright_min_max,
            supported_modes,
            initialised: false,
            flip_effect_write: false,
            _phantom: PhantomData,
        }
    }

    pub async fn do_command(
        &mut self,
        command: AuraCommand,
        config: &mut Config,
    ) -> Result<(), AuraError> {
        if !self.initialised {
            self.write_bytes(&LED_INIT1).await?;
            self.write_bytes(LED_INIT2.as_bytes()).await?;
            self.write_bytes(&LED_INIT3).await?;
            self.write_bytes(LED_INIT4.as_bytes()).await?;
            self.write_bytes(&LED_INIT5).await?;
            self.initialised = true;
        }

        match command {
            AuraCommand::BrightInc => {
                let mut bright = config.brightness;
                if bright < self.bright_min_max.1 {
                    bright += 1;
                    config.brightness = bright;
                    let bytes = aura_brightness_bytes(bright);
                    self.set_and_save(&bytes, config).await?;
                    info!("Increased LED brightness to {:#?}", bright);
                }
            }
            AuraCommand::BrightDec => {
                let mut bright = config.brightness;
                if bright > self.bright_min_max.0 {
                    bright -= 1;
                    config.brightness = bright;
                    let bytes = aura_brightness_bytes(bright);
                    self.set_and_save(&bytes, config).await?;
                    info!("Decreased LED brightness to {:#?}", bright);
                }
            }
            AuraCommand::BuiltinNext => {
                // TODO: different path for multi-zone (byte 2 controlled, non-zero)
                let mode_curr = config.current_mode[3];
                if let Ok(idx) = self.supported_modes.binary_search(&mode_curr.into()) {
                    let idx_next = if idx < self.supported_modes.len() - 1 {
                        idx + 1
                    } else {
                        0
                    };
                    self.set_builtin(config, idx_next).await?;
                } else {
                    warn!("Tried to step to next LED mode while in non-supported mode");
                    self.set_builtin(config, 0).await?;
                }
            }
            AuraCommand::BuiltinPrev => {
                // TODO: different path for multi-zone (byte 2 controlled, non-zero)
                let mode_curr = config.current_mode[3];
                if let Ok(idx) = self.supported_modes.binary_search(&mode_curr.into()) {
                    let idx_next = if idx > 0 {
                        idx - 1
                    } else {
                        self.supported_modes.len() - 1
                    };
                    self.set_builtin(config, idx_next).await?;
                } else {
                    warn!("Tried to step to next LED mode while in non-supported mode");
                    self.set_builtin(config, 0).await?;
                }
            }
            AuraCommand::WriteBytes(bytes) => self.set_and_save(&bytes, config).await?,
            AuraCommand::WriteMultizone(effect) => self.write_multizone(effect).await?,
            AuraCommand::WriteEffect(effect) => self.write_effect(effect).await?,
            AuraCommand::ReloadLast => self.reload_last_builtin(&config).await?,
        }
        Ok(())
    }

    /// Should only be used if the bytes you are writing are verified correct
    #[inline]
    async fn write_bytes(&self, message: &[u8]) -> Result<(), AuraError> {
        match unsafe { self.handle.as_ref() }.write_interrupt(
            self.led_endpoint,
            message,
            Duration::from_millis(2),
        ) {
            Ok(_) => {}
            Err(err) => match err {
                rusb::Error::Timeout => {}
                _ => error!("Failed to write to led interrupt: {:?}", err),
            },
        }
        Ok(())
    }

    #[inline]
    async fn write_array_of_bytes(&self, messages: &[&[u8]]) -> Result<(), AuraError> {
        for message in messages {
            self.write_bytes(*message).await?;
            self.write_bytes(&LED_SET).await?;
        }
        // Changes won't persist unless apply is set
        self.write_bytes(&LED_APPLY).await?;
        Ok(())
    }

    /// Write an effect block
    ///
    /// `aura_effect_init` must be called any effect routine, and called only once.
    #[inline]
    async fn write_effect(&mut self, effect: Vec<Vec<u8>>) -> Result<(), AuraError> {
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

    #[inline]
    async fn write_multizone(&mut self, effect: Vec<Vec<u8>>) -> Result<(), AuraError> {
        for row in effect.iter() {
            self.write_bytes(row).await?;
        }
        self.write_bytes(&LED_SET).await?;
        self.write_bytes(&LED_APPLY).await?;
        Ok(())
    }

    /// Used to set a builtin mode and save the settings for it
    #[inline]
    async fn set_and_save(&self, bytes: &[u8], config: &mut Config) -> Result<(), AuraError> {
        let mode = BuiltInModeByte::from(bytes[3]);
        // safety pass-through of possible effect write
        if bytes[1] == 0xbc {
            self.write_bytes(bytes).await?;
            return Ok(());
        } else if self.supported_modes.contains(&mode) || bytes[1] == 0xba {
            let messages = [bytes];
            self.write_array_of_bytes(&messages).await?;
            config.set_field_from(bytes);
            config.write();
            return Ok(());
        }
        warn!("{:?} not supported", mode);
        Err(AuraError::NotSupported)
    }

    #[inline]
    async fn reload_last_builtin(&self, config: &Config) -> Result<(), AuraError> {
        let mode_curr = config.current_mode[3];
        let mode = config
            .builtin_modes
            .get_field_from(mode_curr)
            .ok_or(AuraError::NotSupported)?
            .to_owned();
        self.write_bytes(&mode).await?;
        // Reload brightness too
        let bright = config.brightness;
        let bytes = aura_brightness_bytes(bright);
        self.write_bytes(&bytes).await?;
        info!("Reloaded last used mode and brightness");
        Ok(())
    }

    #[inline]
    async fn set_builtin(&self, config: &mut Config, index: usize) -> Result<(), AuraError> {
        let mode_next = config
            .builtin_modes
            .get_field_from(self.supported_modes[index].into())
            .ok_or(AuraError::NotSupported)?
            .to_owned();
        println!("{:X?}", &mode_next);
        self.set_and_save(&mode_next, config).await?;
        info!("Switched LED mode to {:#?}", self.supported_modes[index]);
        Ok(())
    }
}
