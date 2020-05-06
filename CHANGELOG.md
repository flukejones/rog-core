# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Flip writing order of effect colour blocks every other block write to try
  and even out. Previously the bottom rows were always last to be written.
- Add more examples: ball, comet, pulser.
- Refine the keyboard layout grid for GX502.

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