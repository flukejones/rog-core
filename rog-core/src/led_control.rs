static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &str = "]ASUS Tech.Inc."; // ] == 0x5d
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0, 0x08];
static LED_INIT4: &str = "^ASUS Tech.Inc."; // ^ == 0x5e
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0, 0x08];

// Only these two packets must be 17 bytes
static LED_APPLY: [u8; 17] = [0x5d, 0xb4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static LED_SET: [u8; 17] = [0x5d, 0xb5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

use crate::{config::Config, error::RogError};
use log::{error, info};
use rog_client::{
    aura_brightness_bytes, aura_modes::AuraModes, fancy::KeyColourArray, LED_MSG_LEN,
};
use rusb::DeviceHandle;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::time::Duration;

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
    supported_modes: Vec<u8>,
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
        supported_modes: Vec<u8>,
    ) -> Self {
        LedWriter {
            handle: device_handle,
            led_endpoint,
            supported_modes,
            initialised: false,
            flip_effect_write: false,
            _phantom: PhantomData,
        }
    }

    async fn initialise(&mut self) -> Result<(), RogError> {
        if !self.initialised {
            self.write_bytes(&LED_INIT1).await?;
            self.write_bytes(LED_INIT2.as_bytes()).await?;
            self.write_bytes(&LED_INIT3).await?;
            self.write_bytes(LED_INIT4.as_bytes()).await?;
            self.write_bytes(&LED_INIT5).await?;
            self.initialised = true;
        }
        Ok(())
    }

    pub async fn do_command(
        &mut self,
        mode: AuraModes,
        config: &mut Config,
    ) -> Result<(), RogError> {
        self.initialise().await?;
        self.set_and_save(mode, config).await
    }

    /// Should only be used if the bytes you are writing are verified correct
    #[inline]
    async fn write_bytes(&self, message: &[u8]) -> Result<(), RogError> {
        match unsafe { self.handle.as_ref() }.write_interrupt(
            self.led_endpoint,
            message,
            Duration::from_millis(5),
        ) {
            Ok(_) => {}
            Err(err) => match err {
                rusb::Error::Timeout => {}
                _ => error!("Failed to write to led interrupt: {:?}", err),
            },
        }
        Ok(())
    }

    /// Write an effect block
    ///
    /// `aura_effect_init` must be called any effect routine, and called only once.
    #[inline]
    async fn write_effect(&mut self, effect: &Vec<Vec<u8>>) -> Result<(), RogError> {
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
                info!(
                    "Switched LED mode to {}",
                    <&str>::from(&<AuraModes>::from(config.current_mode))
                );
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
    pub async fn reload_last_builtin(&mut self, config: &Config) -> Result<(), RogError> {
        self.initialise().await?;
        // set current mode (if any)
        if self.supported_modes.len() > 1 {
            let mode = config
                .get_led_mode_data(config.current_mode)
                .ok_or(RogError::NotSupported)?
                .to_owned();
            let mode: [u8; LED_MSG_LEN] = mode.into();
            self.write_bytes(&mode).await?;
            info!("Reloaded last used mode");
        }

        // Reload brightness
        let bright = config.brightness;
        let bytes = aura_brightness_bytes(bright);
        self.write_bytes(&bytes).await?;
        info!("Reloaded last used brightness");
        Ok(())
    }
}
