# ROG-Core

rog-core is a utility for Linux to control many aspects (eventually) of the ASUS ROG laptops like the Zephyrus GX502GW.

The laptop I currently have is the GX502RW and so I'll be using that for the basis of this app. If I get wireshark captures from others with different ROG laptops then I should be able to add something like laptop and feature detection.

## Requirements

- Rust!
- `libusb-1.0-0-dev` or equivalent package for your distro
- `libdbus-1-dev` or equivalent package for your distro

## Use

Currently requires root for accessing the USB devices (so, sudo).

Commands are given by:

```
rog-core <option> <command> <command-options>
```

Help is available through:

```
rog-core --help
rog-core <command> --help
```

Some commands may have subcommands:

```
rog-core <command> <subcommand> --help
```

## Implemented

- [X] Setting/modifying built-in LED modes
- [ ] Per-key LED setting
- [ ] Fancy LED modes (custom programs)
- [X] Daemon mode (functionally done)
- [ ] Saving settings for reload (CLI and daemon mode)
- [ ] System control
- [ ] Capture and use hotkeys, e.g, Aura controls to control LEDs

## Wireshark captures

TODO

## License

Mozilla Public License 2 (MPL-2.0)