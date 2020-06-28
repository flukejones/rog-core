# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed

### Added
- Dbus signals "FanModeChanged", "ChargeLimitChanged"
- Dbus methods "GetFanMode", "GetChargeLimit", "SetFanMode", "SetChargeLimit"
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
