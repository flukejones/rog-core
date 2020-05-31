use rog_aura::AnimatrixDbusWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AnimatrixDbusWriter::new()?;

    let mut send_buffer = [vec![0; 640], vec![0; 640]];

    loop {
        writer.write_image(&send_buffer)?;

        //std::thread::sleep(std::time::Duration::from_millis(60));
    }
}
