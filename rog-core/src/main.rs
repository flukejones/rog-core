use daemon::daemon::start_daemon;
use env_logger::{Builder, Target};
use gumdrop::Options;
use log::LevelFilter;
use rog_aura::{
    cli_options::{LedBrightness, SetAuraBuiltin},
    AuraDbusWriter, LED_MSG_LEN,
};

static VERSION: &'static str = "0.9.1";

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

    let writer = AuraDbusWriter::new()?;

    if let Some(Command::LedMode(mode)) = parsed.command {
        if let Some(command) = mode.command {
            // Check for special modes here, eg, per-key or multi-zone
            match command {
                SetAuraBuiltin::MultiStatic(_) => {
                    let byte_arr = <[[u8; LED_MSG_LEN]; 4]>::from(command);
                    for arr in byte_arr.iter() {
                        writer.write_bytes(arr)?;
                    }
                }
                _ => {
                    writer.write_builtin_mode(&command)?;
                }
            }
        }
    }
    if let Some(brightness) = parsed.bright {
        writer.write_brightness(brightness.level())?;
    }
    Ok(())
}
