use daemon::daemon::start_daemon;
use daemon::rogcore::FanLevel;
use gumdrop::Options;
use log::info;
use log::LevelFilter;
use rog_client::{
    cli_options::{LedBrightness, SetAuraBuiltin},
    core_dbus::AuraDbusWriter,
};
use std::io::Write;

static VERSION: &str = "0.14.1";

#[derive(Options)]
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

#[derive(Options)]
enum Command {
    #[options(help = "Set the keyboard lighting from built-in modes")]
    LedMode(LedModeCommand),
}

#[derive(Options)]
struct LedModeCommand {
    #[options(help = "print help message")]
    help: bool,
    #[options(command, required)]
    command: Option<SetAuraBuiltin>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut logger = env_logger::Builder::new();
    logger
        .target(env_logger::Target::Stdout)
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();

    let parsed = CLIStart::parse_args_default_or_exit();
    if parsed.daemon {
        info!("Version: {}", VERSION);
        start_daemon().await?;
    }
    if parsed.version {
        println!("Version: {}", VERSION);
    }

    let writer = AuraDbusWriter::new()?;

    if let Some(Command::LedMode(mode)) = parsed.command {
        if let Some(command) = mode.command {
            writer.write_builtin_mode(&command.into())?
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
