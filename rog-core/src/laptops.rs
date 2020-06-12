use crate::{config::Config, led_control::AuraCommand, rogcore::RogCore};
use rog_client::{error::AuraError, BuiltInModeByte};
//use keycode::{KeyMap, KeyMappingId, KeyState, KeyboardState};
use crate::virt_device::ConsumerKeys;
use log::{info, warn};

pub(crate) fn match_laptop() -> LaptopBase {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0b05 {
            match device_desc.product_id() {
                0x1869 | 0x1866 => return choose_1866_device(device_desc.product_id()),
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
                            BuiltInModeByte::Strobe,
                        ],
                        support_animatrix: false,
                        // backlight: Backlight::new("intel_backlight").unwrap(),
                    };
                }
                _ => {}
            }
        }
    }
    panic!("could not match laptop");
}

fn choose_1866_device(prod: u16) -> LaptopBase {
    let dmi = sysfs_class::DmiId::default();
    let board_name = dmi.board_name().expect("Could not get board_name");
    let mut laptop = LaptopBase {
        usb_vendor: 0x0B05,
        usb_product: prod,
        report_filter_bytes: vec![0x5a, 0x02],
        min_led_bright: 0x00,
        max_led_bright: 0x03,
        //from `lsusb -vd 0b05:1866`
        led_endpoint: 0x04,
        //from `lsusb -vd 0b05:1866`
        key_endpoint: 0x83,
        supported_modes: vec![],
        support_animatrix: false,
        //backlight: Backlight::new("intel_backlight").unwrap(),
    };
    match &board_name.as_str()[..5] {
        "GA401" => {
            // Has no RGB control
            info!("Found GA401 series");
            laptop.support_animatrix = true;
        }
        "GA502" => {
            // Has no RGB control
            info!("Found GA502 series");
            laptop.supported_modes = vec![
                BuiltInModeByte::Single,
            ];
        }
        "GX502" => {
            info!("Found GX502 series");
            laptop.supported_modes = vec![
                BuiltInModeByte::Single,
                BuiltInModeByte::Breathing,
                BuiltInModeByte::Strobe,
                BuiltInModeByte::Rainbow,
                BuiltInModeByte::Star,
                BuiltInModeByte::Rain,
                BuiltInModeByte::Highlight,
                BuiltInModeByte::Laser,
                BuiltInModeByte::Ripple,
                BuiltInModeByte::Pulse,
                BuiltInModeByte::Comet,
                BuiltInModeByte::Flash,
            ];
        }
        "GM501" => {
            info!("Found GM501 series");
            laptop.supported_modes = vec![
                BuiltInModeByte::Single,
                BuiltInModeByte::Breathing,
                BuiltInModeByte::Strobe,
                BuiltInModeByte::Rainbow,
            ];
        }
        _ => panic!("Unsupported laptop: {}, please request support at\nhttps://github.com/flukejones/rog-core", board_name),
    }
    laptop
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
    support_animatrix: bool,
    //backlight: Backlight,
}

use tokio::sync::{mpsc, Mutex};

impl LaptopBase {
    /// Pass in LedWriter as Mutex so it is only locked when required
    pub(super) async fn run(
        &self,
        rogcore: &mut RogCore,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
        aura_command: mpsc::Sender<AuraCommand>,
    ) -> Result<(), AuraError> {
        match self.usb_product {
            0x1869 | 0x1866 => {
                self.gx502_runner(rogcore, config, key_buf, aura_command)
                    .await
            }
            0x1854 => {
                self.gl753_runner(rogcore, config, key_buf, aura_command)
                    .await
            }
            _ => panic!("No runner available for this device"),
        }
    }

    pub(super) fn min_led_bright(&self) -> u8 {
        self.min_led_bright
    }
    pub(super) fn max_led_bright(&self) -> u8 {
        self.max_led_bright
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
    pub(super) fn support_animatrix(&self) -> bool {
        self.support_animatrix
    }

    // 0x1866, per-key LEDs, media-keys split from vendor specific
    async fn gx502_runner(
        &self,
        rogcore: &mut RogCore,
        config: &Mutex<Config>,
        key_buf: [u8; 32],
        mut aura_command: mpsc::Sender<AuraCommand>,
    ) -> Result<(), AuraError> {
        match GX502Keys::from(key_buf[1]) {
            GX502Keys::LedBrightUp => {
                aura_command
                    .send(AuraCommand::BrightInc)
                    .await
                    .unwrap_or_else(|err| warn!("LedBrightUp: {}", err));
            }
            GX502Keys::LedBrightDown => {
                aura_command
                    .send(AuraCommand::BrightDec)
                    .await
                    .unwrap_or_else(|err| warn!("LedBrightDown: {}", err));
            }
            GX502Keys::AuraNext => {
                aura_command
                    .send(AuraCommand::BuiltinNext)
                    .await
                    .unwrap_or_else(|_| {});
            }
            GX502Keys::AuraPrevious => {
                aura_command
                    .send(AuraCommand::BuiltinPrev)
                    .await
                    .unwrap_or_else(|_| {});
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
    async fn gl753_runner(
        &self,
        rogcore: &mut RogCore,
        _config: &Mutex<Config>,
        key_buf: [u8; 32],
        mut aura_command: mpsc::Sender<AuraCommand>,
    ) -> Result<(), AuraError> {
        match GL753Keys::from(key_buf[1]) {
            GL753Keys::LedBrightUp => {
                aura_command
                    .send(AuraCommand::BrightInc)
                    .await
                    .unwrap_or_else(|_| {});
            }
            GL753Keys::LedBrightDown => {
                aura_command
                    .send(AuraCommand::BrightDec)
                    .await
                    .unwrap_or_else(|_| {});
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

pub enum GX502Keys {
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
