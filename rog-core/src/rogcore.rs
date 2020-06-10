// Return show-stopping errors, otherwise map error to a log level

use crate::{config::Config, error::RogError, virt_device::VirtKeys};
use log::{error, info, warn};
use rusb::DeviceHandle;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::marker::{PhantomData, PhantomPinned};
use std::path::Path;
use std::process::Command;
use std::ptr::NonNull;
use std::str::FromStr;
use std::time::Duration;

static FAN_TYPE_1_PATH: &str = "/sys/devices/platform/asus-nb-wmi/throttle_thermal_policy";
static FAN_TYPE_2_PATH: &str = "/sys/devices/platform/asus-nb-wmi/fan_boost_mode";
static AMD_BOOST_PATH: &str = "/sys/devices/system/cpu/cpufreq/boost";

/// ROG device controller
///
/// For the GX502GW the LED setup sequence looks like:
///
/// -` LED_INIT1`
/// - `LED_INIT3`
/// - `LED_INIT4`
/// - `LED_INIT2`
/// - `LED_INIT4`
pub struct RogCore {
    handle: DeviceHandle<rusb::GlobalContext>,
    virt_keys: VirtKeys,
    _pin: PhantomPinned,
}

impl RogCore {
    pub fn new(vendor: u16, product: u16, match_endpoint: u8) -> Result<RogCore, Box<dyn Error>> {
        let mut dev_handle = RogCore::get_device(vendor, product).map_err(|err| {
            error!("Could not get keyboard device handle: {:?}", err);
            err
        })?;
        dev_handle.reset()?;
        dev_handle.set_active_configuration(0).unwrap_or(());

        let dev_config = dev_handle.device().config_descriptor(0).map_err(|err| {
            error!("Could not get keyboard device config: {:?}", err);
            err
        })?;
        // Interface with outputs
        let mut interface = 0;
        for iface in dev_config.interfaces() {
            for desc in iface.descriptors() {
                for endpoint in desc.endpoint_descriptors() {
                    if endpoint.address() == match_endpoint {
                        info!("INTERVAL: {:?}", endpoint.interval());
                        info!("MAX_PKT_SIZE: {:?}", endpoint.max_packet_size());
                        info!("SYNC: {:?}", endpoint.sync_type());
                        info!("TRANSFER_TYPE: {:?}", endpoint.transfer_type());
                        info!("ENDPOINT: {:X?}", endpoint.address());
                        info!("INTERFACE: {:X?}", desc.interface_number());
                        interface = desc.interface_number();
                        break;
                    }
                }
            }
        }

        dev_handle
            .set_auto_detach_kernel_driver(true)
            .map_err(|err| {
                error!("Auto-detach kernel driver failed: {:?}", err);
                err
            })?;
        dev_handle.claim_interface(interface).map_err(|err| {
            error!("Could not claim keyboard device interface: {:?}", err);
            err
        })?;

        Ok(RogCore {
            handle: dev_handle,
            virt_keys: VirtKeys::new(),
            _pin: PhantomPinned,
        })
    }

    pub fn virt_keys(&mut self) -> &mut VirtKeys {
        &mut self.virt_keys
    }

    fn get_device(
        vendor: u16,
        product: u16,
    ) -> Result<DeviceHandle<rusb::GlobalContext>, rusb::Error> {
        for device in rusb::devices()?.iter() {
            let device_desc = device.device_descriptor()?;
            if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
                return device.open();
            }
        }
        Err(rusb::Error::NoDevice)
    }

    fn get_fan_path() -> Result<&'static str, std::io::Error> {
        if Path::new(FAN_TYPE_1_PATH).exists() {
            Ok(FAN_TYPE_1_PATH)
        } else if Path::new(FAN_TYPE_2_PATH).exists() {
            Ok(FAN_TYPE_2_PATH)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Fan mode not available",
            ))
        }
    }

    pub async fn fan_mode_reload(&mut self, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let path = RogCore::get_fan_path()?;
        let mut file = OpenOptions::new().write(true).open(path)?;
        file.write_all(format!("{:?}\n", config.fan_mode).as_bytes())
            .unwrap_or_else(|err| error!("Could not write to {}, {:?}", path, err));
        self.set_pstate_for_fan_mode(FanLevel::from(config.fan_mode), config)?;
        info!("Reloaded fan mode: {:?}", FanLevel::from(config.fan_mode));
        Ok(())
    }

    pub fn fan_mode_set(&mut self, n: u8, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let path = RogCore::get_fan_path()?;
        let mut fan_ctrl = OpenOptions::new().read(true).write(true).open(path)?;

        config.fan_mode = n;
        config.write();
        fan_ctrl
            .write_all(format!("{:?}\n", config.fan_mode).as_bytes())
            .unwrap_or_else(|err| error!("Could not write to {}, {:?}", path, err));
        info!("Fan mode set to: {:?}", FanLevel::from(config.fan_mode));
        self.set_pstate_for_fan_mode(FanLevel::from(n), config)?;
        Ok(())
    }

    pub fn fan_mode_step(&mut self, config: &mut Config) -> Result<(), Box<dyn Error>> {
        let mut n = config.fan_mode;
        info!("Current fan mode: {:?}", FanLevel::from(n));
        // wrap around the step number
        if n < 2 {
            n += 1;
        } else {
            n = 0;
        }
        self.fan_mode_set(n, config)
    }

    fn set_pstate_for_fan_mode(
        &self,
        mode: FanLevel,
        config: &mut Config,
    ) -> Result<(), Box<dyn Error>> {
        // re-read the config here in case a user changed the pstate settings
        config.read();
        // Set CPU pstate
        if let Ok(pstate) = intel_pstate::PState::new() {
            info!("Setting pstate for Intel CPU");
            match mode {
                FanLevel::Normal => {
                    pstate.set_min_perf_pct(config.mode_performance.normal.min_percentage)?;
                    pstate.set_max_perf_pct(config.mode_performance.normal.max_percentage)?;
                    pstate.set_no_turbo(config.mode_performance.normal.no_turbo)?;
                    info!(
                        "CPU Power: min-freq: {:?}, max-freq: {:?}, turbo: {:?}",
                        config.mode_performance.normal.min_percentage,
                        config.mode_performance.normal.max_percentage,
                        !config.mode_performance.normal.no_turbo
                    );
                }
                FanLevel::Boost => {
                    pstate.set_min_perf_pct(config.mode_performance.boost.min_percentage)?;
                    pstate.set_max_perf_pct(config.mode_performance.boost.max_percentage)?;
                    pstate.set_no_turbo(config.mode_performance.boost.no_turbo)?;
                    info!(
                        "CPU Power: min-freq: {:?}, max-freq: {:?}, turbo: {:?}",
                        config.mode_performance.boost.min_percentage,
                        config.mode_performance.boost.max_percentage,
                        !config.mode_performance.boost.no_turbo
                    );
                }
                FanLevel::Silent => {
                    pstate.set_min_perf_pct(config.mode_performance.silent.min_percentage)?;
                    pstate.set_max_perf_pct(config.mode_performance.silent.max_percentage)?;
                    pstate.set_no_turbo(config.mode_performance.silent.no_turbo)?;
                    info!(
                        "CPU Power: min-freq: {:?}, max-freq: {:?}, turbo: {:?}",
                        config.mode_performance.silent.min_percentage,
                        config.mode_performance.silent.max_percentage,
                        !config.mode_performance.silent.no_turbo
                    );
                }
            }
        } else {
            info!("Setting pstate for AMD CPU");
            // must be AMD CPU
            let mut file = OpenOptions::new()
                .write(true)
                .open(AMD_BOOST_PATH)
                .map_err(|err| {
                    warn!("Failed to open AMD boost: {:?}", err);
                    err
                })?;
            match mode {
                FanLevel::Normal => {
                    let boost = if config.mode_performance.normal.no_turbo {
                        "0"
                    } else {
                        "1"
                    }; // opposite of Intel
                    file.write_all(boost.as_bytes()).unwrap_or_else(|err| {
                        error!("Could not write to {}, {:?}", AMD_BOOST_PATH, err)
                    });
                    info!("CPU Power: turbo: {:?}", boost);
                }
                FanLevel::Boost => {
                    let boost = if config.mode_performance.boost.no_turbo {
                        "0"
                    } else {
                        "1"
                    };
                    file.write_all(boost.as_bytes()).unwrap_or_else(|err| {
                        error!("Could not write to {}, {:?}", AMD_BOOST_PATH, err)
                    });
                    info!("CPU Power: turbo: {:?}", boost);
                }
                FanLevel::Silent => {
                    let boost = if config.mode_performance.silent.no_turbo {
                        "0"
                    } else {
                        "1"
                    };
                    file.write_all(boost.as_bytes()).unwrap_or_else(|err| {
                        error!("Could not write to {}, {:?}", AMD_BOOST_PATH, err)
                    });
                    info!("CPU Power: turbo: {:?}", boost);
                }
            }
        }
        Ok(())
    }

    /// A direct call to systemd to suspend the PC.
    ///
    /// This avoids desktop environments being required to handle it
    /// (which means it works while in a TTY also)
    pub fn suspend_with_systemd(&self) {
        std::process::Command::new("systemctl")
            .arg("suspend")
            .spawn()
            .map_or_else(|err| warn!("Failed to suspend: {}", err), |_| {});
    }

    /// A direct call to rfkill to suspend wireless devices.
    ///
    /// This avoids desktop environments being required to handle it (which
    /// means it works while in a TTY also)
    pub fn toggle_airplane_mode(&self) {
        match Command::new("rfkill").arg("list").output() {
            Ok(output) => {
                if output.status.success() {
                    if let Ok(out) = String::from_utf8(output.stdout) {
                        if out.contains(": yes") {
                            Command::new("rfkill")
                                .arg("unblock")
                                .arg("all")
                                .spawn()
                                .map_or_else(
                                    |err| warn!("Could not unblock rf devices: {}", err),
                                    |_| {},
                                );
                        } else {
                            Command::new("rfkill")
                                .arg("block")
                                .arg("all")
                                .spawn()
                                .map_or_else(
                                    |err| warn!("Could not block rf devices: {}", err),
                                    |_| {},
                                );
                        }
                    }
                } else {
                    warn!("Could not list rf devices");
                }
            }
            Err(err) => {
                warn!("Could not list rf devices: {}", err);
            }
        }
    }

    pub fn get_raw_device_handle(&mut self) -> NonNull<DeviceHandle<rusb::GlobalContext>> {
        // Breaking every damn lifetime guarantee rust gives us
        unsafe {
            NonNull::new_unchecked(&mut self.handle as *mut DeviceHandle<rusb::GlobalContext>)
        }
    }
}

/// Lifetime is tied to `DeviceHandle` from `RogCore`
pub struct KeyboardReader<'d, C: 'd>
where
    C: rusb::UsbContext,
{
    handle: NonNull<DeviceHandle<C>>,
    endpoint: u8,
    filter: Vec<u8>,
    _phantom: PhantomData<&'d DeviceHandle<C>>,
}

/// UNSAFE
unsafe impl<'d, C> Send for KeyboardReader<'d, C> where C: rusb::UsbContext {}
unsafe impl<'d, C> Sync for KeyboardReader<'d, C> where C: rusb::UsbContext {}

impl<'d, C> KeyboardReader<'d, C>
where
    C: rusb::UsbContext,
{
    pub fn new(device_handle: NonNull<DeviceHandle<C>>, key_endpoint: u8, filter: Vec<u8>) -> Self {
        KeyboardReader {
            handle: device_handle,
            endpoint: key_endpoint,
            filter,
            _phantom: PhantomData,
        }
    }

    /// Write the bytes read from the device interrupt to the buffer arg, and returns the
    /// count of bytes written
    ///
    /// `report_filter_bytes` is used to filter the data read from the interupt so
    /// only the relevant byte array is returned.
    pub async fn poll_keyboard(&self) -> Option<[u8; 32]> {
        let mut buf = [0u8; 32];
        match unsafe { self.handle.as_ref() }.read_interrupt(
            self.endpoint,
            &mut buf,
            Duration::from_millis(200),
        ) {
            Ok(_) => {
                if self.filter.contains(&buf[0])
                    && (buf[1] != 0 || buf[2] != 0 || buf[3] != 0 || buf[4] != 0)
                {
                    return Some(buf);
                }
            }
            Err(err) => match err {
                rusb::Error::Timeout => {}
                _ => error!("Failed to read keyboard interrupt: {:?}", err),
            },
        }
        None
    }
}

#[derive(Debug)]
pub enum FanLevel {
    Normal,
    Boost,
    Silent,
}

impl FromStr for FanLevel {
    type Err = RogError;

    fn from_str(s: &str) -> Result<Self, RogError> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(FanLevel::Normal),
            "boost" => Ok(FanLevel::Boost),
            "silent" => Ok(FanLevel::Silent),
            _ => Err(RogError::ParseFanLevel),
        }
    }
}

impl From<u8> for FanLevel {
    fn from(n: u8) -> Self {
        match n {
            0 => FanLevel::Normal,
            1 => FanLevel::Boost,
            2 => FanLevel::Silent,
            _ => FanLevel::Normal,
        }
    }
}

impl From<FanLevel> for u8 {
    fn from(n: FanLevel) -> Self {
        match n {
            FanLevel::Normal => 0,
            FanLevel::Boost => 1,
            FanLevel::Silent => 2,
        }
    }
}
