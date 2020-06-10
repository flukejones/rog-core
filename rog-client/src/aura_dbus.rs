use super::*;
use dbus::blocking::BlockingSender;
use dbus::channel::Sender;
use dbus::{blocking::Connection, Message};
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
}

impl AuraDbusWriter {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = Connection::new_system()?;
        Ok(AuraDbusWriter {
            connection: Box::new(connection),
            block_time: 33333,
            stop: Arc::new(AtomicBool::new(false)),
        })
    }

    /// This method must always be called before the very first write to initialise
    /// the keyboard LED EC in the correct mode
    #[inline]
    pub fn init_effect(&self) -> Result<String, Box<dyn std::error::Error>> {
        let match_rule = dbus::message::MatchRule::new_signal(DBUS_IFACE, "LedCancelEffect");
        let stopper = self.stop.clone();
        self.connection
            .add_match(match_rule, move |_: (), _, msg| {
                println!("GOT {:?}", msg);
                if let Ok(stop) = msg.read1::<bool>() {
                    if stop {
                        stopper.store(true, Ordering::Relaxed);
                    }
                }
                true
            })?;

        let msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "LedWriteBytes")?
            .append1(KeyColourArray::get_init_msg());
        let r = self
            .connection
            .send_with_reply_and_block(msg, Duration::from_millis(5000))?;
        if let Some(reply) = r.get1::<&str>() {
            return Ok(reply.to_owned());
        }
        Err(Box::new(dbus::Error::new_custom("name", "message")))
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
        if self.stop.load(Ordering::Relaxed) {
            panic!("Got signal to stop!");
        }
        Ok(())
    }

    #[inline]
    pub fn write_multizone(
        &mut self,
        group: &[[u8; LED_MSG_LEN]; 4],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "LedWriteMultizone")?
            .append1(&group[0].to_vec())
            .append1(&group[1].to_vec())
            .append1(&group[2].to_vec())
            .append1(&group[3].to_vec());
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        Ok(())
    }

    #[inline]
    pub fn write_bytes(&self, bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "LedWriteBytes")?
            .append1(bytes.to_vec());
        let r = self
            .connection
            .send_with_reply_and_block(msg, Duration::from_millis(5000))?;
        if let Some(reply) = r.get1::<&str>() {
            return Ok(reply.to_owned());
        }
        Err(Box::new(dbus::Error::new_custom("name", "message")))
    }

    #[inline]
    pub fn write_fan_mode(&self, level: u8) -> Result<String, Box<dyn std::error::Error>> {
        let msg =
            Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "FanMode")?.append1(level);
        let r = self
            .connection
            .send_with_reply_and_block(msg, Duration::from_millis(5000))?;
        if let Some(reply) = r.get1::<&str>() {
            return Ok(reply.to_owned());
        }
        Err(Box::new(dbus::Error::new_custom("name", "message")))
    }

    #[inline]
    pub fn write_builtin_mode(
        &self,
        mode: &SetAuraBuiltin,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = <[u8; LED_MSG_LEN]>::from(mode);
        self.write_bytes(&bytes)
    }

    #[inline]
    pub fn write_brightness(&self, level: u8) -> Result<String, Box<dyn std::error::Error>> {
        self.write_bytes(&aura_brightness_bytes(level))
    }
}
