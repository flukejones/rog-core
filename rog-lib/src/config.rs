use crate::{aura::SetAuraBuiltin, core::LED_MSG_LEN};
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub static CONFIG_PATH: &'static str = "/etc/rogcore.conf";

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub brightness: u8,
    pub builtin: Vec<u8>,
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
                let d = SetAuraBuiltin::default();
                let c = Config {
                    brightness: 1u8,
                    builtin: (<[u8; LED_MSG_LEN]>::from(d)).to_vec(),
                };
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
        let toml = toml::to_string_pretty(self).expect("Parse config to JSON failed");
        file.write_all(toml.as_bytes())
            .expect("Saving config failed");
    }
}
