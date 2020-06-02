# ROG-Core

rog-core is a utility for Linux to control many aspects (eventually) of the ASUS ROG laptops like the Zephyrus GX502GW.

One of the benefits of this app (for me at least) is that you *don't* require a kernel with correct support for the
laptop, or custom patched modules. The app reads and writes direct to the device interrupts, and can be customised
(in source) quite extensively to do what you want such as directly controlling your laptop backlight rather than
emitting a key-press for the DE to handle. There is also the possibility of rebinding fn keys to be macros which emit a
series of keyboard presses.

The laptop I currently have is the GX502RW and so I'll be using that for the basis of this app. If I get wireshark
captures from others with different ROG laptops then I should be able to add them.

I'm now looking at the kernel source to see if I can add the inputs correctly so they show up as proper evdev events.

## Implemented

- [X] Setting/modifying built-in LED modes
- [X] Per-key LED setting (PARTIALLY COMPLETE)
- [X] Fancy LED modes (See examples)
- [X] Daemon mode
- [X] Saving settings for reload
- [ ] System control
  + [X] Sleep
  + [X] Airplane mode
  + [X] Screen off? Now mapped to a keycode but has no effect
  + [X] Screen brightness up/down
  + [ ] ROG key custom mapping (Can be done in source)
  + [X] Fan/Performance mode
  + [ ] Screen off??
  + [X] Touchpad toggle (using a virtual keyboard to emit F21...)
- [X] Capture and use hotkeys **Partially completed: aura keys work**
  + [X] Aura control by Aura keys
  + [X] Volume + media controls work
- [X] Logging - required for journalctl
- [X] AniMatrix display on G14 models (Functional. Needs testing)

## Other Laptops

**Supported:**

- GX502 (product 0x1866) (Tested on GX502GW)

**Partial/Inprogress:**
- GM501 (multizone needs testing)
- G14 (no Lid LED support, all other functions appear to work)

Both of these laptops appear to have the same USB device ID as the GX502, but with different features enabled.

**Please help test or provide info for:**

- GL703(0x1869), GA502 (attempts to use same profile as GX502GW)
- GL553(0x1854) GL753 (attempted support from researching 2nd-hand info, multizone may work)

If the USB product ID is 0x1866 or 0x1869 then the per-key profile with hotkeys *should* work - 0x1866 is tested as this
is what I have.

## Requirements for compiling

- `rustc` + `cargo` + `make`
- `libusb-1.0-0-dev`
- `libdbus-1-dev`
- `llvm`
- `libclang-dev`

## Installing

Run `make` then `sudo make install`. If you want to use the daemon mode on system boot you'll need to enable and start
the systemd service with:

```
$ sudo systemctl start rog-core.service
$ sudo systemctl enable rog-core.service
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
sudo install rog-core
```

enable and start the service:

```
sudo systemctl start rog-core.service
sudo systemctl enable rog-core.service
```

### Gentoo ebuild

Thanks to @aspann you can grab it here [sys-power/rog-core](https://lab.retarded.farm/zappel/zGentoo/-/tree/master/sys-power/rog-core)

## Updating

Occasionally I might break things for you by tweaking or changing the config file layout. Usually this will mean you
need to remove `/etc/rog-core.conf' and restart the daemon to create a new one. You *can* back up the old one and copy
settings back over (then restart daemon again).

## Use

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

## Daemon mode

If the daemon service is enabled then on boot the following will be reloaded from save:

- LED brightness
- Last used built-in mode
- fan-boost/thermal mode 

The daemon also saves the settings per mode as the keyboard does not do this itself - this means cycling through modes
with the Aura keys will use the settings that were used via CLI.

Daemon mode creates a config file at `/etc/rogcore.conf` which you can edit a little of. Most parts will be byte arrays,
but you can adjust things like `mode_performance`.

### DBUS Input

Paths:

```rust
pub static DBUS_NAME: &str = "org.rogcore.Daemon";
pub static DBUS_PATH: &str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &str = "org.rogcore.Daemon";
```

Commands: `FanMode`, `LedWriteBytes`, `LedWriteMultizone`, `LedWriteEffect`

TODO: fill in this info

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
- Anyone missed? Please contact me