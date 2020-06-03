use super::*;
use dbus::channel::Sender;
use dbus::{blocking::Connection, Message};
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

pub const ANIME_PANE1_PREFIX: [u8; 7] = [0x5e, 0xc0, 0x02, 0x01, 0x00, 0x73, 0x02];
pub const ANIME_PANE2_PREFIX: [u8; 7] = [0x5e, 0xc0, 0x02, 0x74, 0x02, 0x73, 0x02];

/// Interface for the AniMe dot-matrix display
///
/// The resolution is 34x56 (1904) but only 1,215 LEDs in the top-left are used.
/// The display is available only on select GA401 models.
///
/// Amory crate assumes first row is 33 pixels/bytes, this means the actual structure
/// likely follows this format with every second line offset physically by half.
/// Even rows (mod 1) are aligned right, odd are offset left by half (`..=` is inclusive)
///
/// -   0..=32,  row 0, 33 pixels // len starts at 37 if using formula
/// -  33..=66,  row 1, 33 pixels
/// -  68..=101, row 2, 33 pixels
/// - 101..=134, row 3, 33 pixels
/// - 135..=168, row 4, 33 pixels
/// - 169..=202, row 5, 33 pixels // Should be last offset line?
/// - 203..=236, row 6, 33 pixels
/// - 237..=268, row 7, 31 pixels -2 px from last
/// - 269..=301, row 8, 32 pixels +1 px from last
/// - 302..=332, row 9, 30 pixels -2
/// - 333..=364, row 10, 31 pixels +1
/// - 365..=394, row 11, 29 pixels -2
/// - 395..=425, row 12, 30 pixels +1
/// - 426..=454, row 13, 28 pixels -2
/// - 455..=484, row 14, 29 pixels +1
/// - 485..=512, row 15, 27 pixels -2
/// - 513..=541, row 16, 28 pixels +1
/// - 542..=568, row 17, 26 pixels -2
/// - 569..=596, row 18, 27 pixels +1
/// - 597..=622, row 19, 25 pixels -2
/// - BEGIN NEXT BLOCK AT IDX627 (when writing out the one dimensional array)
/// - 623..=649, row 20, 26 pixels +1
/// - .. 57 rows (from 0)
///
/// Image is 33x56, and
///
/// The formula below starts at row 7
/// ```
///  if current_row_idx != 0 && current_row_idx.mod(1) == 0
///  then current_row_len = last_row_len + 1
///  else current_row_len = last_row_len - 2, // offset left by half
/// ```
///
/// Data structure should be nested array of [[u8; 33]; 56]
pub struct AniMeDbusWriter {
    connection: Box<Connection>,
    block_time: u64,
    stop: Arc<AtomicBool>,
}

impl AniMeDbusWriter {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = Connection::new_system()?;
        Ok(AniMeDbusWriter {
            connection: Box::new(connection),
            block_time: 25,
            stop: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn write_image_to_buf(_buf: &mut AniMePacketType, _image_data: &[u8]) {
        unimplemented!("Image format is in progress of being worked out")
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
    /// - Write packet 1: 0x5e 0xc0 0x02 0x01 0x00 0x73 0x02 .. <led brightness>
    /// - Write packet 2: 0x5e 0xc0 0x02 0x74 0x02 0x73 0x02 .. <led brightness>
    ///
    /// Where led brightness is 0..255, low to high
    #[inline]
    pub fn write_image(&mut self, image: &mut AniMePacketType) -> Result<(), Box<dyn Error>> {
        if image[0][0] != ANIME_PANE1_PREFIX[0] && image[0][6] != ANIME_PANE1_PREFIX[6] {
            image[0][..7].copy_from_slice(&ANIME_PANE1_PREFIX);
        }
        if image[1][0] != ANIME_PANE2_PREFIX[0] && image[1][6] != ANIME_PANE2_PREFIX[6] {
            image[1][..7].copy_from_slice(&ANIME_PANE2_PREFIX);
        }

        let mut msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "AnimatrixWrite")?
            .append2(image[0].to_vec(), image[1].to_vec());
        msg.set_no_reply(true);
        self.connection.send(msg).unwrap();
        thread::sleep(Duration::from_millis(self.block_time));
        if self.stop.load(Ordering::Relaxed) {
            panic!("Got signal to stop!");
        }
        Ok(())
    }
}
