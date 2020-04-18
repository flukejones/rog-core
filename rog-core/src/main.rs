mod daemon;

use crate::daemon::*;
use dbus::Error as DbusError;
use dbus::{ffidisp::Connection, Message};
use env_logger::{Builder, Target};
use gumdrop::Options;
use log::LevelFilter;
use rog_lib::{
    cli_options::SetAuraBuiltin,
    core::{LedBrightness, RogCore, LED_MSG_LEN},
};

pub static DBUS_NAME: &'static str = "org.rogcore.Daemon";
pub static DBUS_PATH: &'static str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &'static str = "org.rogcore.Daemon";

#[derive(Debug, Options)]
struct CLIStart {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "start daemon")]
    daemon: bool,
    #[options(meta = "VAL", help = "<off, low, med, high>")]
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
    let mut builder = Builder::from_env("ROGCORE_LOG");
    builder.target(Target::Stdout);
    builder.format_timestamp(None);
    builder.filter(None, LevelFilter::Info).init();

    let parsed = CLIStart::parse_args_default_or_exit();
    if parsed.daemon {
        Daemon::start()?;
    }

    match parsed.command {
        Some(Command::LedMode(mode)) => {
            if let Some(command) = mode.command {
                let mode = <[u8; LED_MSG_LEN]>::from(command);
                dbus_led_builtin_write(&mode)?;
            }
        }
        None => {}
    }
    match parsed.bright {
        Some(brightness) => {
            let bytes = RogCore::aura_brightness_bytes(brightness.level())?;
            dbus_led_builtin_write(&bytes)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn dbus_led_builtin_write(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let bus = Connection::new_system()?;
    //let proxy = bus.with_proxy(DBUS_IFACE, "/", Duration::from_millis(5000));
    let msg = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_IFACE, "ledmessage")?
        .append1(bytes.to_vec());
    let r = bus.send_with_reply_and_block(msg, 5000)?;
    if let Some(reply) = r.get1::<&str>() {
        println!("Success: {:x?}", reply);
        return Ok(());
    }
    Err(Box::new(DbusError::new_custom("name", "message")))
}
