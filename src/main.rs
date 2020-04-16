// TODO: use /sys/class/dmi/id/board_name to detect model
mod aura;
mod core;
mod daemon;
mod error;

use crate::aura::*;
use crate::core::{dbus_led_builtin_write, LedBrightness, RogCore};
use crate::daemon::*;
use gumdrop::Options;

pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

#[derive(Debug, Options)]
struct CLIStart {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "start daemon")]
    daemon: bool,
    #[options(help = "client mode")]
    client: bool,
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
    if parsed.daemon {
        start_daemon()?;
    }

    match parsed.command {
        Some(Command::LedMode(okk)) => match okk.command {
            Some(command) => {
                let mode = ModeMessage::from(command);
                if parsed.client {
                    dbus_led_builtin_write(&mode.0)?;
                } else {
                    let mut core = RogCore::new()?;
                    core.aura_set_mode(&mode.0)?;
                }
            }
            _ => {}
        },
        _ => {}
    }
    match parsed.bright {
        Some(brightness) => {
            let bytes = RogCore::aura_brightness_bytes(brightness.level as u8)?;
            if parsed.client {
                dbus_led_builtin_write(&bytes)?;
            } else {
                let mut core = RogCore::new()?;
                core.aura_set_mode(&bytes)?;
            }
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
