use super::*;
use dbus::channel::Sender;
use dbus::{blocking::Connection, Message};
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

/// Simplified way to write a effect block
pub struct AnimatrixDbusWriter {
    connection: Box<Connection>,
    block_time: u64,
    stop: Arc<AtomicBool>,
}

impl AnimatrixDbusWriter {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = Connection::new_system()?;
        Ok(AnimatrixDbusWriter {
            connection: Box::new(connection),
            block_time: 25,
            stop: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Write an Animatrix image
    ///
    /// The expected input here is *two* Vectors, 640 bytes in length. The two vectors
    /// are each one half of the full image write.
    ///
    /// After each write a flush is written, it is assumed that this tells the device to
    /// go ahead and display the written bytes
    ///
    /// # Note:
    /// The vectors are expected to contain the full sequence of bytes as follows
    ///
    /// - Write pane 1: 0x5e 0xc0 0x02 0x01 0x00 0x73 0x02 .. <led brightness>
    /// - Write pane 2: 0x5e 0xc0 0x02 0x74 0x02 0x73 0x02 .. <led brightness>
    ///
    /// Where led brightness is 0..255, low to high
    #[inline]
    pub fn write_image(&mut self, image: &[Vec<u8>; 2]) -> Result<(), Box<dyn Error>> {
        let msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "AnimatrixWrite")?
            .append2(image[0].to_vec(), image[1].to_vec());
        self.connection.send(msg).unwrap();
        thread::sleep(Duration::from_millis(self.block_time));
        if self.stop.load(Ordering::Relaxed) {
            panic!("Got signal to stop!");
        }
        Ok(())
    }
}
