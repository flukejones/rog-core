use daemon::daemon::start_daemon;
use daemon::rogcore::FanLevel;
use env_logger::{Builder, Target};
use gumdrop::Options;
use log::LevelFilter;
use rog_client::{
    aura_modes::AuraModes,
    cli_options::{LedBrightness, SetAuraBuiltin},
    core_dbus::AuraDbusWriter,
    LED_MSG_LEN,
};

static VERSION: &str = "0.12.2";

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
    #[options(meta = "FAN", help = "<silent, normal, boost>")]
    fan_mode: Option<FanLevel>,
    #[options(meta = "CHRG", help = "<20-100>")]
    charge_limit: Option<u8>,
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
    builder
        .target(Target::Stdout)
        .format_module_path(false)
        .format_timestamp(None)
        .filter(None, LevelFilter::Info)
        .init();

    let parsed = CLIStart::parse_args_default_or_exit();
    if parsed.daemon {
        start_daemon().await?;
    }
    if parsed.version {
        println!("Version: {}", VERSION);
    }

    let mut writer = AuraDbusWriter::new()?;

    if let Some(Command::LedMode(mode)) = parsed.command {
        if let Some(command) = mode.command {
            // Check for special modes here, eg, per-key or multi-zone
            match command {
                SetAuraBuiltin::MultiStatic(_) => {
                    let command: AuraModes = command.into();
                    let byte_arr = <[[u8; LED_MSG_LEN]; 4]>::from(command);
                    writer.write_multizone(&byte_arr)?;
                }
                _ => writer.write_builtin_mode(&command.into())?,
            }
        }
    }
    if let Some(brightness) = parsed.bright {
        writer.write_brightness(brightness.level())?;
    }
    if let Some(fan_level) = parsed.fan_mode {
        writer.write_fan_mode(fan_level.into())?;
    }
    if let Some(charge_limit) = parsed.charge_limit {
        writer.write_charge_limit(charge_limit)?;
    }
    Ok(())
}
