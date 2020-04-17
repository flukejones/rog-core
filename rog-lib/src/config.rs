use crate::core::LED_MSG_LEN;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub static CONFIG_PATH: &'static str = "/etc/rogcore.conf";

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub brightness: u8,
    pub current_mode: [u8; 4],
    stable: [u8; LED_MSG_LEN],
    breathe: [u8; LED_MSG_LEN],
    cycle: [u8; LED_MSG_LEN],
    rainbow: [u8; LED_MSG_LEN],
    rain: [u8; LED_MSG_LEN],
    random: [u8; LED_MSG_LEN],
    highlight: [u8; LED_MSG_LEN],
    laser: [u8; LED_MSG_LEN],
    ripple: [u8; LED_MSG_LEN],
    pulse: [u8; LED_MSG_LEN],
    thinzoomy: [u8; LED_MSG_LEN],
    widezoomy: [u8; LED_MSG_LEN],
}

impl Config {
    pub fn read(mut self) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&CONFIG_PATH)
            .expect("config file error");
        let mut buf = String::new();
        if let Ok(l) = file.read_to_string(&mut buf) {
            if l == 0 {
                // create a default config here
                let c = Config::default();
                let toml = toml::to_string(&c).unwrap();
                file.write_all(toml.as_bytes())
                    .expect("Writing default config failed");
                self = c;
            } else {
                self = toml::from_str(&buf).unwrap();
            }
        }
        self
    }

    pub fn write(&self) {
        let mut file = File::create(CONFIG_PATH).expect("Couldn't overwrite config");
        let toml = toml::to_string(self).expect("Parse config to JSON failed");
        file.write_all(toml.as_bytes())
            .expect("Saving config failed");
    }

    pub fn set_field_from(&mut self, bytes: &[u8]) {
        if bytes[0] == 0x5a && bytes[1] == 0xba {
            self.brightness = bytes[4];
        } else if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            self.current_mode.copy_from_slice(&bytes[0..4]);

            match bytes[3] {
                0x00 => self.stable.copy_from_slice(bytes),
                0x01 => self.breathe.copy_from_slice(bytes),
                0x02 => self.cycle.copy_from_slice(bytes),
                0x03 => self.rainbow.copy_from_slice(bytes),
                0x04 => self.rain.copy_from_slice(bytes),
                0x05 => self.random.copy_from_slice(bytes),
                0x06 => self.highlight.copy_from_slice(bytes),
                0x07 => self.laser.copy_from_slice(bytes),
                0x08 => self.ripple.copy_from_slice(bytes),
                0x0a => self.pulse.copy_from_slice(bytes),
                0x0b => self.thinzoomy.copy_from_slice(bytes),
                0x0c => self.widezoomy.copy_from_slice(bytes),
                _ => {}
            }
        }
    }

    pub fn get_current(&mut self) -> Option<Vec<u8>> {
        let bytes = self.current_mode;
        if bytes[0] == 0x5d && bytes[1] == 0xb3 {
            let bytes = match bytes[3] {
                0x00 => &self.stable,
                0x01 => &self.breathe,
                0x02 => &self.cycle,
                0x03 => &self.rainbow,
                0x04 => &self.rain,
                0x05 => &self.random,
                0x06 => &self.highlight,
                0x07 => &self.laser,
                0x08 => &self.ripple,
                0x0a => &self.pulse,
                0x0b => &self.thinzoomy,
                0x0c => &self.widezoomy,
                _ => return None,
            };
            return Some(bytes.to_vec());
        }
        None
    }
}
