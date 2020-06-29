use super::*;
use crate::fancy::KeyColourArray;
use dbus::channel::Sender;
use dbus::{blocking::Connection, channel::Token, Message};
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

/// Simplified way to write a effect block
pub struct AuraDbusWriter {
    connection: Box<Connection>,
    block_time: u64,
    stop: Arc<AtomicBool>,
    stop_token: Token,
}

impl AuraDbusWriter {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = Connection::new_system()?;
        let stop = Arc::new(AtomicBool::new(false));

        let stopper2 = stop.clone();
        let match_rule = dbus::message::MatchRule::new_signal(DBUS_IFACE, "KeyBacklightChanged");
        let stop_token = connection.add_match(match_rule, move |_: (), _, msg| {
            dbg!(&msg);
            if let Ok(stop) = msg.read1::<bool>() {
                if stop {
                    stopper2.store(true, Ordering::Relaxed);
                }
            }
            true
        })?;

        Ok(AuraDbusWriter {
            connection: Box::new(connection),
            block_time: 33333,
            stop,
            stop_token,
        })
    }

    /// This method must always be called before the very first write to initialise
    /// the keyboard LED EC in the correct mode
    #[inline]
    pub fn init_effect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mode = AuraModes::Aura;
        let mut msg =
            Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "SetKeyBacklight")?
                .append1(serde_json::to_string(&mode)?);
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    /// Write a single colour block.
    ///
    /// Intentionally blocks for 10ms after sending to allow the block to
    /// be written to the keyboard EC. This should not be async.
    #[inline]
    pub fn write_colour_block(
        &mut self,
        key_colour_array: &KeyColourArray,
    ) -> Result<(), Box<dyn Error>> {
        let group = key_colour_array.get();
        let mut msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "LedWriteEffect")?
            .append3(&group[0].to_vec(), &group[1].to_vec(), &group[2].to_vec())
            .append3(&group[3].to_vec(), &group[4].to_vec(), &group[5].to_vec())
            .append3(&group[6].to_vec(), &group[7].to_vec(), &group[8].to_vec())
            .append2(&group[9].to_vec(), &group[10].to_vec());
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        thread::sleep(Duration::from_micros(self.block_time));
        self.connection.process(Duration::from_micros(500))?;
        if self.stop.load(Ordering::Relaxed) {
            self.connection.remove_match(self.stop_token)?;
            println!("Keyboard backlight was changed, exiting");
            std::process::exit(1)
        }
        Ok(())
    }

    #[inline]
    pub fn write_multizone(
        &mut self,
        group: &[[u8; LED_MSG_LEN]; 4],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg =
            Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "LedWriteMultizone")?
                .append1(&group[0].to_vec())
                .append1(&group[1].to_vec())
                .append1(&group[2].to_vec())
                .append1(&group[3].to_vec());
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    #[inline]
    pub fn write_bytes(&self, mode: &AuraModes) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg =
            Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "SetKeyBacklight")?
                .append1(serde_json::to_string(mode)?);
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    #[inline]
    pub fn write_fan_mode(&self, level: u8) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "SetFanMode")?
            .append1(level);
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    #[inline]
    pub fn write_charge_limit(&self, level: u8) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "SetChargeLimit")?
            .append1(level);
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    #[inline]
    pub fn write_builtin_mode(&self, mode: &AuraModes) -> Result<(), Box<dyn std::error::Error>> {
        self.write_bytes(mode)
    }

    #[inline]
    pub fn write_brightness(&self, level: u8) -> Result<String, Box<dyn std::error::Error>> {
        self.write_bytes(&AuraModes::LedBrightness(level))?;
        Ok(String::new())
    }
}
