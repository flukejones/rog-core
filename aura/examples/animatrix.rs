use rog_aura::AniMeDbusWriter;

fn main() {
    let mut writer = AniMeDbusWriter::new().unwrap();

    loop {
        for brightness in 0..0xFF {
            let mut buffers = [[brightness; 640], [brightness; 640]];
            writer.write_image(&mut buffers).unwrap();
        }
    }
}
