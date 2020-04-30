use crate::aura::BuiltInModeByte;
use crate::config::Config;
use crate::core::{LedWriter, RogCore};
use crate::error::AuraError;
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use crate::virt_device::ConsumerKeys;
use log::{info, warn};

pub(crate) fn match_laptop() -> LaptopBase {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0b05 {
            match device_desc.product_id() {
                0x1869 | 0x1866 => {
                    info!("Found GX502 or similar");
                    return LaptopBase {
                        usb_vendor: 0x0B05,
                        usb_product: 0x1866,
                        report_filter_bytes: vec![0x5a, 0x02],
                        min_led_bright: 0x00,
                        max_led_bright: 0x03,
                        //from `lsusb -vd 0b05:1866`
                        led_endpoint: 0x04,
                        //from `lsusb -vd 0b05:1866`
                        key_endpoint: 0x83,
                        supported_modes: vec![
                            BuiltInModeByte::Single,
                            BuiltInModeByte::Breathing,
                            BuiltInModeByte::Cycle,
                            BuiltInModeByte::Rainbow,
                            BuiltInModeByte::Rain,
                            BuiltInModeByte::Random,
                            BuiltInModeByte::Highlight,
                            BuiltInModeByte::Laser,
                            BuiltInModeByte::Ripple,
                            BuiltInModeByte::Pulse,
                            BuiltInModeByte::ThinZoomy,
                            BuiltInModeByte::WideZoomy,
                        ],
                        //backlight: Backlight::new("intel_backlight").unwrap(),
                    };
                }
                0x1854 => {
                    info!("Found GL753 or similar");
                    return LaptopBase {
                        usb_vendor: 0x0B05,
                        usb_product: 0x1854,
                        report_filter_bytes: vec![0x5a, 0x02],
                        min_led_bright: 0x00,
                        max_led_bright: 0x03,
                        //from `lsusb -vd 0b05:1866`
                        led_endpoint: 0x04,
                        //from `lsusb -vd 0b05:1866`
                        key_endpoint: 0x83,
                        supported_modes: vec![
                            BuiltInModeByte::Single,
                            BuiltInModeByte::Breathing,
                            BuiltInModeByte::Cycle,
                        ],
                        // backlight: Backlight::new("intel_backlight").unwrap(),
                    };
                }
                _ => {}
            }
        }
    }
    panic!("could not match laptop");
}

pub(super) struct LaptopBase {
    usb_vendor: u16,
    usb_product: u16,
    report_filter_bytes: Vec<u8>,
    min_led_bright: u8,
    max_led_bright: u8,
    led_endpoint: u8,
    key_endpoint: u8,
    supported_modes: Vec<BuiltInModeByte>,
    //backlight: Backlight,
}

use tokio::sync::Mutex;

impl LaptopBase {
    /// Pass in LedWriter as Mutex so it is only locked when required
    pub(super) async fn run<'a, C>(
        &self,
        rogcore: &mut RogCore,
        led_writer: &Mutex<LedWriter<'a, C>>,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
    ) -> Result<(), AuraError>
    where
        C: rusb::UsbContext,
    {
        match self.usb_product {
            0x1869 | 0x1866 => {
                self.gx502_runner(rogcore, led_writer, config, key_buf)
                    .await
            }
            0x1854 => {
                self.gl753_runner(rogcore, led_writer, config, key_buf)
                    .await
            }
            _ => panic!("No runner available for this device"),
        }
    }

    pub(super) fn led_endpoint(&self) -> u8 {
        self.led_endpoint
    }
    pub(super) fn key_endpoint(&self) -> u8 {
        self.key_endpoint
    }
    pub(super) fn key_filter(&self) -> &[u8] {
        &self.report_filter_bytes
    }
    pub(super) fn usb_vendor(&self) -> u16 {
        self.usb_vendor
    }
    pub(super) fn usb_product(&self) -> u16 {
        self.usb_product
    }
    pub(super) fn supported_modes(&self) -> &[BuiltInModeByte] {
        &self.supported_modes
    }

    // 0x1866, per-key LEDs, media-keys split from vendor specific
    async fn gx502_runner<'a, C>(
        &self,
        rogcore: &mut RogCore,
        led_writer: &Mutex<LedWriter<'a, C>>,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
    ) -> Result<(), AuraError>
    where
        C: rusb::UsbContext,
    {
        let max_led_bright = self.max_led_bright;
        let min_led_bright = self.min_led_bright;
        let supported_modes = self.supported_modes.to_owned();
        match GX502Keys::from(key_buf[1]) {
            GX502Keys::LedBrightUp => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_bright_inc(&supported_modes, max_led_bright, &mut config)
                    .await?;
            }
            GX502Keys::LedBrightDown => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_bright_dec(&supported_modes, min_led_bright, &mut config)
                    .await?;
            }
            GX502Keys::AuraNext => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_mode_next(&supported_modes, &mut config)
                    .await?;
            }
            GX502Keys::AuraPrevious => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_mode_prev(&supported_modes, &mut config)
                    .await?;
            }
            GX502Keys::ScreenBrightUp => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightInc.into())
            } //self.backlight.step_up(),
            GX502Keys::ScreenBrightDown => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightDec.into())
            } //self.backlight.step_down(),
            GX502Keys::Sleep => rogcore.suspend_with_systemd(),
            GX502Keys::AirplaneMode => rogcore.toggle_airplane_mode(),
            GX502Keys::MicToggle => {}
            GX502Keys::Fan => {
                let mut config = config.lock().await;
                rogcore.fan_mode_step(&mut config).unwrap_or_else(|err| {
                    warn!("Couldn't toggle fan mode: {:?}", err);
                });
            }
            GX502Keys::ScreenToggle => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
            }
            GX502Keys::TouchPadToggle => {
                let mut key = [0u8; 32];
                key[0] = 0x01;
                key[3] = 0x070;
                rogcore.virt_keys().press(key);
            }
            GX502Keys::Rog => {
                //rogcore.aura_effect_init()?;
                //rogcore.aura_write_effect(&self.per_key_led)?;
                let mut key = [0u8; 32];
                key[0] = 0x01;
                key[3] = 0x68; // XF86Tools? F13
                rogcore.virt_keys().press(key);
            }
            GX502Keys::None => {
                if key_buf[0] != 0x5A {
                    info!("Unmapped key, attempt passthrough: {:X?}", &key_buf[1]);
                    rogcore.virt_keys().press(key_buf);
                }
            }
        }
        Ok(())
    }

    // GL753VE == 0x1854, 4 zone keyboard
    async fn gl753_runner<'a, C>(
        &self,
        rogcore: &mut RogCore,
        led_writer: &Mutex<LedWriter<'a, C>>,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
    ) -> Result<(), AuraError>
    where
        C: rusb::UsbContext,
    {
        let max_led_bright = self.max_led_bright;
        let min_led_bright = self.min_led_bright;
        let supported_modes = self.supported_modes.to_owned();
        match GL753Keys::from(key_buf[1]) {
            GL753Keys::LedBrightUp => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_bright_inc(&supported_modes, max_led_bright, &mut config)
                    .await?;
            }
            GL753Keys::LedBrightDown => {
                let mut led_writer = led_writer.lock().await;
                let mut config = config.lock().await;
                led_writer
                    .aura_bright_dec(&supported_modes, min_led_bright, &mut config)
                    .await?;
            }
            GL753Keys::ScreenBrightUp => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightInc.into())
            }
            GL753Keys::ScreenBrightDown => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightDec.into())
            }
            GL753Keys::Sleep => rogcore.suspend_with_systemd(),
            GL753Keys::AirplaneMode => rogcore.toggle_airplane_mode(),
            GL753Keys::ScreenToggle => {
                rogcore.virt_keys().press(ConsumerKeys::BacklightTog.into());
            }
            GL753Keys::TouchPadToggle => {
                let mut key = [0u8; 32];
                key[0] = 0x01;
                key[3] = 0x070;
                rogcore.virt_keys().press(key);
            }
            GL753Keys::Rog => {
                let mut key = [0u8; 32];
                key[0] = 0x01;
                key[3] = 0x68; // XF86Tools? F13
                rogcore.virt_keys().press(key);
            }
            GL753Keys::None => {
                if key_buf[0] != 0x5A {
                    info!("Unmapped key, attempt passthrough: {:X?}", &key_buf[1]);
                    rogcore.virt_keys().press(key_buf);
                }
            }
        }
        Ok(())
    }
}

pub(crate) enum GX502Keys {
    Rog = 0x38,
    MicToggle = 0x7C,
    Fan = 0xAE,
    ScreenToggle = 0x35,
    ScreenBrightDown = 0x10,
    ScreenBrightUp = 0x20,
    TouchPadToggle = 0x6b,
    Sleep = 0x6C,
    AirplaneMode = 0x88,
    LedBrightUp = 0xC4,
    LedBrightDown = 0xC5,
    AuraPrevious = 0xB2,
    AuraNext = 0xB3,
    None,
}

impl From<u8> for GX502Keys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => GX502Keys::Rog,
            0x7C => GX502Keys::MicToggle,
            0xAE => GX502Keys::Fan,
            0x35 => GX502Keys::ScreenToggle,
            0x10 => GX502Keys::ScreenBrightDown,
            0x20 => GX502Keys::ScreenBrightUp,
            0x6b => GX502Keys::TouchPadToggle,
            0x6C => GX502Keys::Sleep,
            0x88 => GX502Keys::AirplaneMode,
            0xC4 => GX502Keys::LedBrightUp,
            0xC5 => GX502Keys::LedBrightDown,
            0xB2 => GX502Keys::AuraPrevious,
            0xB3 => GX502Keys::AuraNext,
            _ => GX502Keys::None,
        }
    }
}

enum GL753Keys {
    Rog = 0x38,
    ScreenToggle = 0x35,
    ScreenBrightDown = 0x10,
    ScreenBrightUp = 0x20,
    TouchPadToggle = 0x6b,
    Sleep = 0x6C,
    AirplaneMode = 0x88,
    LedBrightUp = 0xC4,
    LedBrightDown = 0xC5,
    None,
}

impl From<u8> for GL753Keys {
    fn from(byte: u8) -> Self {
        match byte {
            0x38 => GL753Keys::Rog,
            0x35 => GL753Keys::ScreenToggle,
            0x10 => GL753Keys::ScreenBrightDown,
            0x20 => GL753Keys::ScreenBrightUp,
            0x6b => GL753Keys::TouchPadToggle,
            0x6C => GL753Keys::Sleep,
            0x88 => GL753Keys::AirplaneMode,
            0xC4 => GL753Keys::LedBrightUp,
            0xC5 => GL753Keys::LedBrightDown,
            _ => GL753Keys::None,
        }
    }
}
