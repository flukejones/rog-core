use crate::aura::BuiltInModeBytes;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub static CONFIG_PATH: &str = "/etc/rogcore.conf";

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub fan_mode: u8,
    pub brightness: u8,
    pub current_mode: [u8; 4],
    pub builtin_modes: BuiltInModeBytes,
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
                let mut c = Config::default();
                c.current_mode[0] = 0x5d;
                c.current_mode[1] = 0xb3;
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
            self.builtin_modes.set_field_from(bytes);
        }
    }
}
