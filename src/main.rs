use daemon::{
    aura::aura_brightness_bytes,
    cli_options::SetAuraBuiltin,
    core::{LedBrightness, LED_MSG_LEN},
    daemon::{start_daemon, DBUS_IFACE, DBUS_NAME, DBUS_PATH},
};
use dbus::Error as DbusError;
use dbus::{ffidisp::Connection, Message};
use env_logger::{Builder, Target};
use gumdrop::Options;
use log::LevelFilter;

static VERSION: &'static str = "0.7.0";

#[derive(Debug, Options)]
struct CLIStart {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "show program version number")]
    version: bool,
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

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    builder.format_timestamp(None);
    builder.filter(None, LevelFilter::Info).init();

    let parsed = CLIStart::parse_args_default_or_exit();
    if parsed.daemon {
        start_daemon().await?;
    }
    if parsed.version {
        println!("Version: {}", VERSION);
    }

    if let Some(Command::LedMode(mode)) = parsed.command {
        if let Some(command) = mode.command {
            // Check for special modes here, eg, per-key or multi-zone
            match command {
                SetAuraBuiltin::MultiStatic(_) => {
                    let byte_arr = <[[u8; LED_MSG_LEN]; 4]>::from(command);
                    for arr in byte_arr.iter() {
                        dbus_led_builtin_write(arr)?;
                    }
                }
                _ => {
                    let mode = <[u8; LED_MSG_LEN]>::from(command);
                    dbus_led_builtin_write(&mode)?;
                }
            }
        }
    }
    if let Some(brightness) = parsed.bright {
        let bytes = aura_brightness_bytes(brightness.level());
        dbus_led_builtin_write(&bytes)?;
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
