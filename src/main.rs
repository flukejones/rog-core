// TODO: use /sys/class/dmi/id/board_name to detect model
mod aura;
mod core;

use crate::aura::*;
use crate::core::{LedBrightness, RogCore};
use gumdrop::Options;

#[derive(Debug, Options)]
struct CLIStart {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "<off, low, med, high>")]
    bright: Option<LedBrightness>,
    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "Set the keyboard lighting from built-in modes")]
    LedMode(LedModeCommand),
}

#[derive(Debug, Options)]
struct LedModeCommand {
    #[options(help = "print help message")]
    help: bool,
    #[options(command, required)]
    command: Option<SetAuraBuiltin>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = CLIStart::parse_args_default_or_exit();
    match parsed.command {
        Some(Command::LedMode(okk)) => match okk.command {
            Some(command) => {
                let mut core = RogCore::new()?;
                let mode = ModeMessage::from(command);
                core.aura_set_mode(mode)?;
            }
            _ => {}
        },
        _ => {}
    }
    match parsed.bright {
        Some(brightness) => {
            let mut core = RogCore::new()?;
            core.aura_set_brightness(brightness.level as u8)?;
        }
        _ => {}
    }

    //core.aura_set_brightness(3)?;

    //   core.test_builtins(aura::LED_SPEED_BYTES[2])?;

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
