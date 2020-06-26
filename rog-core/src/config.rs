use rog_client::BuiltInModeBytes;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub static CONFIG_PATH: &str = "/etc/rogcore.conf";

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub fan_mode: u8,
    pub bat_charge_limit: u8,
    pub brightness: u8,
    pub current_mode: [u8; 4],
    pub builtin_modes: BuiltInModeBytes,
    pub mode_performance: FanModeSettings,
}

impl Config {
    /// `load` will attempt to read the config, but if it is not found it
    /// will create a new default config and write that out.
    pub fn load(mut self) -> Self {
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
                c.bat_charge_limit = 100;
                c.current_mode[0] = 0x5d;
                c.current_mode[1] = 0xb3;
                // Should be okay to unwrap this as is since it is a Default
                let toml = toml::to_string(&c).unwrap();
                file.write_all(toml.as_bytes())
                    .unwrap_or_else(|_| panic!("Could not deserialise {}", CONFIG_PATH));
                self = c;
            } else {
                self = toml::from_str(&buf)
                    .unwrap_or_else(|_| panic!("Could not deserialise {}", CONFIG_PATH));
            }
        }
        self
    }

    pub fn read(&mut self) {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&CONFIG_PATH)
            .expect("config file error");
        let mut buf = String::new();
        if let Ok(l) = file.read_to_string(&mut buf) {
            if l == 0 {
                panic!("Missing {}", CONFIG_PATH);
            } else {
                let x: Config = toml::from_str(&buf)
                    .unwrap_or_else(|_| panic!("Could not deserialise {}", CONFIG_PATH));
                *self = x;
            }
        }
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

#[derive(Default, Deserialize, Serialize)]
pub struct FanModeSettings {
    pub normal: IntelPState,
    pub boost: IntelPState,
    pub silent: IntelPState,
}

#[derive(Deserialize, Serialize)]
pub struct IntelPState {
    pub min_percentage: u8,
    pub max_percentage: u8,
    pub no_turbo: bool,
}

impl Default for IntelPState {
    fn default() -> Self {
        IntelPState {
            min_percentage: 0,
            max_percentage: 100,
            no_turbo: false,
        }
    }
}
