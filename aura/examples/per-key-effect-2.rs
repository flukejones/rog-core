use rog_aura::{AuraDbusWriter, Key, KeyColourArray};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AuraDbusWriter::new()?;

    let mut per_key_led = Vec::new();
    let mut key_colours = KeyColourArray::new();
    per_key_led.push(key_colours.clone());

    for _ in 0..49 {
        *key_colours.key(Key::ROG).0 += 3;
        *key_colours.key(Key::L).0 += 3;
        *key_colours.key(Key::I).0 += 3;
        *key_colours.key(Key::N).0 += 3;
        *key_colours.key(Key::U).0 += 3;
        *key_colours.key(Key::X).0 += 3;
        per_key_led.push(key_colours.clone());
    }

    for _ in 0..49 {
        *key_colours.key(Key::ROG).0 -= 3;
        *key_colours.key(Key::L).0 -= 3;
        *key_colours.key(Key::I).0 -= 3;
        *key_colours.key(Key::N).0 -= 3;
        *key_colours.key(Key::U).0 -= 3;
        *key_colours.key(Key::X).0 -= 3;
        per_key_led.push(key_colours.clone());
    }

    writer.init_effect()?;
    loop {
        let now = std::time::Instant::now();
        for group in &per_key_led {
            writer.write_colour_block(group)?;
        }
        dbg!(std::time::Instant::now().duration_since(now).as_millis());
        //return Ok(());
    }
}
