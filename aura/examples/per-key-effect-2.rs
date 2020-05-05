use rog_aura::{AuraDbusWriter, Key, KeyColourArray};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AuraDbusWriter::new()?;

    let mut key_colours = KeyColourArray::new();

    writer.init_effect()?;
    loop {
        let count = 49;
        for _ in 0..count {
            *key_colours.key(Key::ROG).0 += 5;
            *key_colours.key(Key::L).0 += 5;
            *key_colours.key(Key::I).0 += 5;
            *key_colours.key(Key::N).0 += 5;
            *key_colours.key(Key::U).0 += 5;
            *key_colours.key(Key::X).0 += 5;
            writer.write_colour_block(&key_colours)?;
        }
        for _ in 0..count {
            *key_colours.key(Key::ROG).0 -= 5;
            *key_colours.key(Key::L).0 -= 5;
            *key_colours.key(Key::I).0 -= 5;
            *key_colours.key(Key::N).0 -= 5;
            *key_colours.key(Key::U).0 -= 5;
            *key_colours.key(Key::X).0 -= 5;
            writer.write_colour_block(&key_colours)?;
        }
    }
}
