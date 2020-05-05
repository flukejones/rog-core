use rog_aura::{AuraDbusWriter, GX502Layout, Key, KeyColourArray, KeyLayout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AuraDbusWriter::new()?;

    let mut key_colours = KeyColourArray::new();
    let layout = GX502Layout::default();

    writer.init_effect()?;
    let rows = layout.get_rows();
    loop {
        for (r, row) in rows.iter().enumerate() {
            for (k, key) in row.iter().enumerate() {
                *key_colours.key(*key).0 = 254;
                // Last key of previous row
                if k == 0 {
                    if r == 0 {
                        let k = &rows[rows.len() - 1][rows[rows.len() - 1].len() - 1];
                        *key_colours.key(*k).0 = 0;
                    } else {
                        let k = &rows[r - 1][rows[r - 1].len() - 1];
                        *key_colours.key(*k).0 = 0;
                    }
                } else {
                    let k = &rows[r][k - 1];
                    *key_colours.key(*k).0 = 0;
                }
                *key_colours.key(Key::Up).0 = 255;
                *key_colours.key(Key::Left).0 = 255;
                *key_colours.key(Key::Right).0 = 255;
                *key_colours.key(Key::Down).0 = 255;

                *key_colours.key(Key::W).0 = 255;
                *key_colours.key(Key::A).0 = 255;
                *key_colours.key(Key::S).0 = 255;
                *key_colours.key(Key::D).0 = 255;

                writer.write_colour_block(&key_colours)?;
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
