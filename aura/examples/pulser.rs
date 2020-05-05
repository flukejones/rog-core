use rog_aura::{AuraDbusWriter, GX502Layout, Key, KeyColourArray, KeyLayout};
use std::ops::Sub;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AuraDbusWriter::new()?;

    let mut key_colours = KeyColourArray::new();
    let layout = GX502Layout::default();

    writer.init_effect()?;
    let rows = layout.get_rows();

    let mut fade = 50;
    let mut flip = false;
    loop {
        for row in rows {
            for (k, key) in row.iter().enumerate() {
                *key_colours.key(*key).1 = 255 / fade / (k + 1) as u8;
            }
        }

        writer.write_colour_block(&key_colours)?;

        if flip {
            if fade > 1 {
                fade -= 1;
            } else {
                flip = !flip;
            }
        } else if fade < 50 {
            fade += 1;
        } else {
            flip = !flip;
        }
    }
}
