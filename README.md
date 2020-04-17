# ROG-Core

rog-core is a utility for Linux to control many aspects (eventually) of the ASUS ROG laptops like the Zephyrus GX502GW.

The laptop I currently have is the GX502RW and so I'll be using that for the basis of this app. If I get wireshark captures from others with different ROG laptops then I should be able to add something like laptop and feature detection.

## Requirements

- `rustc`, `cargo`
- `libusb-1.0-0-dev` or equivalent package for your distro
- `libdbus-1-dev` or equivalent package for your distro

## Installing

Run `make` then `sudo make install`. If you want to use the daemon mode on system boot you'll need to enable and start the systemd service with:

```
$ sudo systemctl start rog-core.service
$ sudo systemctl enable rog-core.service
```

## Use

Running the program in standard mode (not daemon or client) will require sudo.
Otherwise if running in daemon mode (option `-d`) then you will need to add option `-c` before all commands to send the command to the daemon - this will change to default to sending to the daemon in future.

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

## Daemon mode

Currently the last used brightness and builtin mode will be saved when set, and loaded when the daemon is started. The effect here is the last settings used are the ones loaded on boot. The daemon also saves the settings per mode as the keyboard does not do this itself - this means cycling through modes with the Aura keys will use the settings that were used via CLI.

Currently if no options are supplied for the CLI mode selection then a default is used, and this will overwrite the saved setting. I will fix this at a later date.

## Implemented

- [X] Setting/modifying built-in LED modes
- [ ] Per-key LED setting
- [ ] Fancy LED modes (custom programs)
- [X] Daemon mode
- [X] Saving settings for reload
- [ ] System control
- [X] Capture and use hotkeys **Partially completed: aura keys work**
- [ ] Logging - required for journalctl

## Wireshark captures

TODO: see `./wireshark_data/` for some captures.

## License

Mozilla Public License 2 (MPL-2.0)