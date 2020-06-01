use rog_aura::AnimatrixDbusWriter;

const PANE1_PREFIX: [u8; 7] = [0x5e, 0xc0, 0x02, 0x01, 0x00, 0x73, 0x02];
const PANE2_PREFIX: [u8; 7] = [0x5e, 0xc0, 0x02, 0x74, 0x02, 0x73, 0x02];

fn main() {
    let mut writer = AnimatrixDbusWriter::new().unwrap();

    loop {
        for brightness in 0..0xFF {
            let mut buffers = [vec![brightness; 640], vec![brightness; 640]];
            buffers[0][..7].copy_from_slice(&PANE1_PREFIX);
            buffers[0][..7].copy_from_slice(&PANE2_PREFIX);

            writer.write_image(&buffers).unwrap();
        }
    }
}
