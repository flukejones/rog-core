# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.14.4] - 2020-07-07
### Changed
- Changes to the way rog-core is initialised

## [0.14.3] - 2020-05-07
### Changed
- Proper fix of laptop mode conversion bug

## [0.14.2] - 2020-04-07
### Changed
- Try to correct how laptop modes are handled

## [0.14.1] - 2020-04-07
### Added
- Support for G512 series
- Support for GU502 (variant of GA15)

## [0.14.0] - 2020-01-07
### Changed
- Further refine the way setting LED modes works so it is more universal, this
  also allows for setting the brightness through `SetKeyBacklight`

### Added
- `README_DBUS.md` to document DBUS interface
- Dbus method `GetKeyBacklight`
- Dbus method `GetKeyBacklightModes`

## [0.13.1] - 2020-29-06
### Fixed
- Properly initialise the device
- Better log formatting

## [0.13.0] - 2020-29-06
### Changed
- Dbus command `LedWriteBytes` renamed to `SetKeyBacklight`
- Dbus command `SetKeyBacklight` will now take a JSON string instead of an array of bytes
  + Example:
    ```
    {
      "Breathe": {
        "help": false,
        "colour": [
          254,
          5,
          171
        ],
        "colour2": [
          0,
          156,
          156
        ],
        "speed": "High"
      }
    }
    ```
    note that `help` is unused but required as a side-effect of using `gumdrop`
    for CLI.
- Dbus signal `LedCancelEffect` renamed to `KeyBacklightChanged`
- Config file is now in JSON format, and will store only the LED modes that a laptop
  supports rather than the whole set

## [0.12.2] - 2020-29-06
### Changed
- "FanMode", "ChargeLimit" to "SetFanMode", "SetChargeLimit"
 
### Added
- Dbus signals "FanModeChanged", "ChargeLimitChanged"
- Dbus methods "GetFanMode", "GetChargeLimit"
- Support for ROG Strix G712

## [0.12.0] - 2020-26-06
### Changed
- Add modes for FX531 LEDs
- Change where USB reset is called
- Add support for G531GT
- Remove duplicated code: it looks like there is at least *some* consistency in Consumer-Device keycodes that ASUS uses

### BREAKING CHANGE
- `bat_charge_limit = 100` must be appended to the top of `/etc/rogcore.conf`

## [0.11.1] - 2020-11-06
### Changed
- Use DBUS_NAME instead of DBUS_IFACE when requesting the name
- Give different names for the bytearray arguments so that pydbus is not confused
- Add org.freedesktop.DBus.Method.NoReply annotations
- Remove dbus reply for multizone writes
- Move led_writer to a main loop due to strange mpsc behaviour, this combined with
  mpsc channels offers a more reactive app
- Small fix for unreported mode indexing issue

## [0.11.0] - 2020-09-06
### BREAKING CHANGE
- Many of the RGB built-in modes have been renamed to match Armory-Crate names,
  this means that the `/etc/rogcore.conf` needs to be removed so it can be
  regenerated using the updated names.

### Changed
- Add better examples for AniMe display, and fix many small issues
- Don't halt app iff AniMe display not found when running on GA14/GA15 laptops
- Rename client crate to better suit
- Added a device reset for both keyboard and AniMe devices before configuring them

## [0.10.0] - 2020-23-05
### Changed
- Correctly set AMD boost
- Add animatrix support for G14 laptops

## [0.9.7] - 2020-23-05
### Changed
- Start differentiating between models using the 0x1866 USB device
- Refactor how to send multizone over dbus, and how to write it (write 4 packets before writing SET/APPLY)
- Begin implementing profiles per board_name
- Boost toggle for AMD (not freq adjustment yet)

## [0.9.6] - 2020-22-05
### Changed
- Fix  needing to double-tap fan-mode to change mode

## [0.9.5] - 2020-22-05
### Changed
- Flip writing order of effect colour blocks every other block write to try
  and even out. Previously the bottom rows were always last to be written.
- Add more examples: ball, comet, pulser.
- Refine the keyboard layout grid for GX502.
- Use tokio to delay main loop, not thread::sleep
- Minor tweaks to error handling. Needs refactor of errors.

## [0.9.4] - 2020-05-05
### Changed
- Fix reloading last keyboard brightness on boot

## [0.9.3] - 2020-05-04
### Changed
- Fixed return of rog-core in client mode
- Fixed writing of customised builtin LED modes

## [0.9.2] - 2020-05-04
### Added
- Begin keeping a Changelog

### Changed
- Internal structure changes to reduce the possibility of mutex await deadlocks when
  writing to the LED endpoint
