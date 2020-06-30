# DBUS Guide

```rust
pub static DBUS_NAME: &str = "org.rogcore.Daemon";
pub static DBUS_PATH: &str = "/org/rogcore/Daemon";
pub static DBUS_IFACE: &str = "org.rogcore.Daemon";
```

## Methods

- `SetKeyBacklight`
- `AnimatrixWrite`
- `SetFanMode`
- `GetFanMode`
- `SetChargeLimit`
- `GetChargeLimit`
- TODO: GetLEDModes

## Signals

- `KeyBacklightChanged`
- `FanModeChanged`
- `ChargeLimitChanged`

## Method Inputs

### SetKeyBacklight

This method expects a string of JSON as input. The JSON is of format such:

```
{
  "Stable": {
    "colour": [ 255, 0, 0]
  }
}
```

The possible contents of a mode are:

- `"colour": [u8, u8, u8],`
- `"speed": <String>,` <Low, Med, High>
- `"direction": <String>,` <Up, Down, Left, Right>

Modes may or may not be available for a specific laptop (TODO: dbus getter for
supported modes). Modes are:

- `"Stable": { "colour": <colour> },`
- `"Pulse": { "colour": <colour> },`
- `"Comet": { "colour": <colour> },`
- `"Flash": { "colour": <colour> },`
- `"Strobe": { "speed": <speed> },`
- `"Rain": { "speed": <speed> },`
- `"Laser": { "colour": <colour>, "speed": <speed> },`
- `"Ripple": { "colour": <colour>, "speed": <speed> },`
- `"Highlight": { "colour": <colour>, "speed": <speed> },`
- `"Rainbow": { "direction": <direction>, "speed": <speed> },`
- `"Breathe": { "colour": <colour>, "colour2": <colour>, "speed": <speed> },`
- `"Star": { "colour": <colour>, "colour2": <colour>, "speed": <speed> },`
- `"MultiStatic": { "colour1": <colour>, "colour2": <colour>, , "colour3": <colour>, "colour4": <colour> },`

Additionally to the above there is `"RGB": [[u8; 64]; 11]` which is for per-key
setting of LED's but this requires some refactoring to make it easily useable over
dbus.

Lastly, there is `"LedBrightness": <u8>` which accepts 0-3 for off, low, med, high.

### AnimatrixWrite

Used to write data to the AniMe display if available. Currently is takes `[[u8; 640]; 2]`
which must be the byte data that will be written directly to the USB device.

### SetFanMode

Accepts an integer from the following:

- `0`: Normal
- `1`: Boost mode
- `2`: Silent mode

### GetFanMode

Returns the integer set from above.

### SetChargeLimit

Accepts an integer in the range of 20-100.

### GetChargeLimit

Returns the integer set from above.

## Signal Outs

### KeyBacklightChanged

When emitted, it will emit a true or false.

### FanModeChanged

When emitted, it will include the integer the fan mode was changed to.

### ChargeLimitChanged

When emitted, it will include the integer the charging limit was changed to.