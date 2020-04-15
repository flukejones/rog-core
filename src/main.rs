mod aura;
mod core;

use crate::aura::{BuiltInMode, ModeMessage};
use crate::core::RogCore;
use gumdrop::{Options, ParsingStyle};
use std::env;

#[derive(Debug, Options)]
struct CLIStart {
    #[options(command, required)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    #[options(help = "show help for a command (eg: help aura)")]
    Help(FreeOptions),
    #[options(help = "Set the keyboard lighting from built-in modes")]
    LedMode(BuiltInMode),
}

#[derive(Debug, Options)]
struct FreeOptions {
    #[options(free)]
    free: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match CLIStart::parse_args(&args[1..], ParsingStyle::AllOptions) {
        Ok(okk) => {
            dbg!(&okk);
            match okk.command {
                Some(Command::Help(help)) => {
                    if help.free.contains(&"aura".to_string()) {
                        println!("\n{}", BuiltInMode::usage())
                    }
                }
                Some(Command::LedMode(aura)) => {
                    let mut core = RogCore::new()?;
                    let mode = ModeMessage::from(aura);
                    core.aura_set_mode(mode)?;
                }
                None => {}
            }
        }
        Err(err) => {
            println!("\n{}", err);
            println!("\nUsage:\n{}", Command::usage());
        }
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
