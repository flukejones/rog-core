# ROG-Core

rog-core is a utility for Linux to control many aspects (eventually) of the ASUS
ROG laptops like the Zephyrus GX502GW.

One of the benefits of this app (for me at least) is that you *don't* require a
kernel with correct support for the laptop keyboard EC. The app
reads and writes direct to the device interrupts, and can be customised (in
source) quite extensively to do what you want such as directly controlling your
laptop backlight rather than emitting a key-press for the DE to handle. There is
also the possibility of rebinding fn keys to be macros which emit a series of
keyboard presses.

Other laptop functions such as fan modes or battery charge limiting will need
kernel level support.

## Discord

[Discord server link](https://discord.gg/PVyFzWj)

## SUPPORTED LAPTOPS

- GM501
- GX502
- GX531
- G512
- G712
- GX531
- G531
- GA14/GA401 *is* supported, including the AniMe display. You will need kernel [patches](https://lab.retarded.farm/zappel/asus-rog-zephyrus-g14/-/tree/master/kernel_patches).
- GA15/GA502/GU502 appears to have most things working

**Please help test or provide info for:**

- GL703(0x1869)
- GL553/GL753 (device = 0x1854) (attempted support from researching 2nd-hand info, multizone may work)

**Laptop support is added on a per-case basis** as the EC for the keyboard varies
a little between models, e.g, some RGB modes are missing, or it's a single colour.
As far as I can see, the EC does not give us a way to find what modes are supported.

## Implemented

- [X] Setting/modifying built-in LED modes
- [X] Per-key LED setting (PARTIALLY COMPLETE)
- [X] Fancy LED modes (See examples)
- [X] Daemon mode
- [X] Saving settings for reload
- [ ] System control
  + [X] ROG key custom mapping (Can be done in source)
  + [X] Fan/Performance mode
  + [ ] Screen off? Now mapped to a keycode but has no effect
  + [X] Screen brightness up/down
  + [X] Touchpad toggle (using a virtual keyboard to emit F21...)
  + [X] Sleep
  + [X] Airplane mode
- [X] Capture and use hotkeys
  + [X] Aura control by Aura keys
  + [X] Volume + media controls work
  + [X] Mic mute - unsure which key should be emitted for this to work. The key by itself emits a code.
- [X] Logging - required for journalctl
- [X] AniMatrix display on G14 models that include it
- [X] Set battery charge limit (with kernel supporting this)

## Requirements for compiling

- `rustc` + `cargo` + `make`
- `libusb-1.0-0-dev`
- `libdbus-1-dev`
- `llvm`
- `libclang-dev`

## Installing

Run `make` then `sudo make install` then reboot.

The default init method is to use the udev rule, this ensures that the service is
started when the device is initialised and ready.

If you are upgrading from a previous installed version, you will need to restart the service or reboot.

```
$ systemctl daemon-reload && systemctl restart rog-core
```

You may also need to activate the service for debian install. If running Pop!_OS, I suggest disabling `system76-power`
gnome-shell extension, or at least limiting use of the power-management parts as `rog-core` lets you set the same things
(one or the other will overwrite pstates). I will create a shell extension at some point similar to system76, but using
the rog-core parts. It is safe to leave `system76-power.service` enabled and use for switching between graphics modes.

### Ubuntu PPA

Alternatively, instead of building manually you can use the PPA.

```
sudo add-apt-repository ppa:lukedjones/rog-core
sudo apt-get update
sudo apt-get install rog-core
```

the rog-core service will run when the device is initialised.

### Gentoo ebuild

Thanks to @aspann you can grab it here [sys-power/rog-core](https://lab.retarded.farm/zappel/zGentoo/-/tree/master/sys-power/rog-core)

### Arch AUR

You can find the thingimajig [here](https://aur.archlinux.org/packages/rog-core/). If
you are using a GA14 or GA15 series, you may need kernel [patches](https://lab.retarded.farm/zappel/asus-rog-zephyrus-g14/-/tree/master/kernel_patches).

## Updating

Occasionally I might break things for you by tweaking or changing the config file layout. Usually this will mean you
need to remove `/etc/rog-core.conf` and restart the daemon to create a new one. You *can* back up the old one and copy
settings back over (then restart daemon again).

# Usage

**NOTE! Fan mode toggling requires a newer kernel**. I'm unsure when the patches required for it got merged - I've
tested with the 5.6.6 kernel and above only. To see if the fan-mode changed cat either:

- `cat /sys/devices/platform/asus-nb-wmi/throttle_thermal_policy` or
- `cat /sys/devices/platform/asus-nb-wmi/fan_boost_mode`

The numbers are 0 = Normal/Balanced, 1 = Boost, 2 = Silent.

Running the program as a daemon manually will require root. Standard (non-daemon) mode expects to be communicating with
the daemon mode over dbus.

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

### Example

```
$ rog-core --help
Usage: rog-core [OPTIONS]

Optional arguments:
  -h, --help          print help message
  -v, --version       show program version number
  -d, --daemon        start daemon
  -b, --bright VAL    <off, low, med, high>
  -f, --fan-mode FAN  <silent, normal, boost>

Available commands:
  led-mode  Set the keyboard lighting from built-in modes

$ rog-core led-mode --help
Usage: rog-core led-mode [OPTIONS]

Optional arguments:
  -h, --help  print help message

Available commands:
  stable        set a single static colour
  breathe       pulse between one or two colours
  strobe        strobe through all colours
  rainbow       rainbow cycling in one of four directions
  star          rain pattern mimicking raindrops
  rain          rain pattern of three preset colours
  highlight     pressed keys are highlighted to fade
  laser         pressed keys generate horizontal laser
  ripple        pressed keys ripple outwards like a splash
  pulse         set a rapid pulse
  comet         set a vertical line zooming from left
  flash         set a wide vertical line zooming from left
  multi-static  4-zone multi-colour

$ rog-core led-mode stable --help
Usage: rog-core led-mode stable [OPTIONS]

Optional arguments:
  -h, --help  print help message
  -c HEX      set the RGB value e.g, ff00ff

$ rog-core led-mode star --help
Usage: rog-core led-mode star [OPTIONS]

Optional arguments:
  -h, --help  print help message
  -c HEX      set the first RGB value e.g, ff00ff
  -C HEX      set the second RGB value e.g, ff00ff
  -s SPEED    set the speed: low, med, high
```

## ROG Key

The ROG key can be customized in a limited way by use of presets:
- Power
- Sleep
- MediaRecord
- MediaFastFwd
- MediaRewind
- MediaNext
- MediaPrev
- MediaStop
- MediaPlayPause
- MediaVolMute
- MediaVolUp
- MediaVolDown
- BacklightInc
- BacklightDec
- ControlConfig
- LaunchTextEditor
- LaunchEmailApp
- LaunchNewsReader
- LaunchCalendar
- LaunchCalculator
- LaunchWebBrowser
- FileBrowser

This key can be changed in /etc/rogcore.conf, for example:

```
{
  "rog_key": "FileBrowser",
```

## Daemon mode

If the daemon service is enabled then on boot the following will be reloaded from save:

- LED brightness
- Last used built-in mode
- fan-boost/thermal mode
- battery charging limit

The daemon also saves the settings per mode as the keyboard does not do this
itself - this means cycling through modes with the Aura keys will use the
settings that were used via CLI.

Daemon mode creates a config file at `/etc/rogcore.conf` which you can edit a 
little of. Most parts will be byte arrays, but you can adjust things like
`mode_performance`.

### DBUS Input

See [README_DBUS.md](./README_DBUS.md).

### AniMe input

You will want to look at what MeuMeu has done with [https://github.com/Meumeu/ZephyrusBling/](https://github.com/Meumeu/ZephyrusBling/)

### Wireshark captures

TODO: see `./wireshark_data/` for some captures.

### Supporting more laptops

At a minimum it probably needs to be a Zephyrus laptop. If there is enough interest I will remove the restriction on
board names so that anyone can try the app.

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

```text
Bus 001 Device 005: ID 0b05:1866 ASUSTek Computer, Inc. N-KEY Device
```

Then do `sudo lsusb -vd 0b05:1866 > ~/laptop_info` and give that to me.

Other helpful info can be gained from `sudo usbhid-dump`, for which you may need to unload kernel drivers. Please google
this.

Also required (for my book-keeping of data):
- `cat /sys/class/dmi/id/product_name`
- `cat /sys/class/dmi/id/product_family`
- `cat /sys/class/dmi/id/board_name`

## License

Mozilla Public License 2 (MPL-2.0)

# Credits

- [flukejones](https://github.com/flukejones/), project maintainer.
- [tuxuser](https://github.com/tuxuser/)
- [aspann](https://github.com/aspann)
- [meumeu](https://github.com/Meumeu)
- Anyone missed? Please contact me
