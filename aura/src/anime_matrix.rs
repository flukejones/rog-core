pub const WIDTH: usize = 34; // Width is definitely 34 items
pub const HEIGHT: usize = 56;
pub type AniMeBufferType = [[u8; WIDTH]; HEIGHT];
pub type AniMePacketType = [[u8; 640]; 2];
const BLOCK_START: usize = 7;
const BLOCK_END: usize = 634;

pub struct AniMeMatrix(AniMeBufferType);

impl AniMeMatrix {
    pub fn new() -> Self {
        AniMeMatrix([[0u8; WIDTH]; HEIGHT])
    }

    pub fn get(&self) -> &AniMeBufferType {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut AniMeBufferType {
        &mut self.0
    }

    pub fn fill_with(&mut self, fill: u8) {
        for row in self.0.iter_mut() {
            for x in row.iter_mut() {
                *x = fill;
            }
        }
    }
}

impl From<AniMeMatrix> for AniMePacketType {
    /// Do conversion from the nested Vec in AniMeMatrix to the two required
    /// packets suitable for sending over USB
    #[inline]
    fn from(anime: AniMeMatrix) -> Self {
        let mut buffers = [[0; 640]; 2];

        let mut write_index = BLOCK_START;
        let mut write_block = &mut buffers[0];
        let mut block1_done = false;
        let mut phys_row_len = WIDTH - 1; // not taking in to account starting at 0

        for (count, row) in anime.0.iter().enumerate() {
            // Write the top block of LEDs (first 7 rows)
            if count < 6 {
                for x in row.iter() {
                    write_block[write_index] = *x;
                    write_index += 1;
                }
            } else {
                // Two offsets to correct the below with
                if count == 6 {
                    phys_row_len -= 1;
                }
                if count == 7 {
                    phys_row_len += 1;
                }

                // Switch to next block (looks like )
                if count % 2 != 0 {
                    phys_row_len -= 2; // if count 7, -= 1
                } else {
                    phys_row_len += 1; // if count 6, 0
                }

                let index = row.len() - phys_row_len;
                for n in index..row.len() {
                    // Require a special case to catch the correct end-of-packet which is
                    // 6 bytes from the end
                    if write_index == BLOCK_END && !block1_done {
                        block1_done = true;
                        write_block = &mut buffers[1];
                        write_index = BLOCK_START;
                    }

                    //println!("{:?}", write_block.to_vec());
                    write_block[write_index] = row[n];
                    write_index += 1;
                }
            }
        }
        buffers
    }
}
