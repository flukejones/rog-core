extern crate rusb;
mod aura;

use aura::RogCore;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut core = RogCore::new()?;
    core.aura_set_brightness(0)?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    core.aura_set_brightness(3)?;

    core.aura_set_colour((0xff, 0x00, 0x00))?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    core.test_builtins(aura::LED_SPEED_BYTES[2])?;
    //core.aura_test()?;

    // std::thread::sleep(std::time::Duration::from_secs(5));
    // core.aura_set_rainbow(aura::LED_SPEED_BYTES[2])?;

    // // READ KEYBOARD
    // let mut buf = [0; 32];
    // loop {
    //     match handle.read_interrupt(0x83, &mut buf, Duration::new(0, 5)) {
    //         Ok(_) => println!("{:?}", buf),
    //         Err(err) => match err {
    //             Error::Timeout => {}
    //             _ => return Err(Box::new(err)),
    //         },
    //     }
    // }
    Ok(())
}
