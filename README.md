# ROG-Core

rog-core is a utility for Linux to control many aspects (eventually) of the ASUS ROG laptops like the Zephyrus GX502GW.

The laptop I currently have is the GX502RW and so I'll be using that for the basis of this app. If I get wireshark captures from others with different ROG laptops then I should be able to add something like laptop and feature detection.

I'm now looking at the kernel source to see if I can add the inputs correctly so they show up as proper evdev events.

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

Running the program as a daemon manually will require root. Standard (non-daemon) mode expects to be communicating with the daemon mode over dbus.

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
  + [X] Sleep
  + [X] Airplane mode
  + [X] Screen off? Now mapped to a keycode but has no effect
  + [X] Screen brightness up/down
  + [ ] ROG key custom mapping (Can be done)
  + [ ] Fan/Performance mode
  + [ ] Screen off??
  + [ ] Touchpad toggle
- [X] Capture and use hotkeys **Partially completed: aura keys work**
  + [X] Aura control by Aura keys
  + [X] Volume + media controls work
- [X] Logging - required for journalctl

As the daemon currently stands it should be enough for a functional system.

## Wireshark captures

TODO: see `./wireshark_data/` for some captures.

## Supporting more laptops

At a minimum it probably needs to be a Zephyrus laptop. If there is enough interest I will remove the restriction on board names so that anyone can try the app.

From there I'll need wireshark captures from Windows using Armoury Crate if possible:

- *No* USB devices plugged in
- One action per capture, 2 or 3 times
- An action may be switching a mode or pressing a button

The real ticket items are:

- Captures from installing Armoury Crate. To do this you need to uninstall almost all ASUS junkware and such
- Byte sequence captures from hotkeys pressed in linux (which I need to make a tool for)
- lsusb output

lsusb:

First do `lsusb |grep 0b05` and check the part after `0b05:`, output looks like:

```asm
Bus 001 Device 005: ID 0b05:1866 ASUSTek Computer, Inc. N-KEY Device
```

Then do `lsusb -vd 0b05:1866 > ~/laptop_info` and give that to me.

`cat /sys/class/dmi/id/product_name` is the other I'm interested in for the sake of information.

## License

Mozilla Public License 2 (MPL-2.0)
