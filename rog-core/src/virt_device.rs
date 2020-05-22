use log::error;
use uhid_virt::{Bus, CreateParams, UHIDDevice};

/// Create a virtual device to emit key-presses
///
/// This is required in some instances because either the USB device that
/// an interface for a working set of buttons is also captured, or because
/// there is no equivalent "system" action to take for a key function and
/// a key-press is required to emit a particular key code.
///
/// The two devices set up mirror that of the GX502GW and can accept the same
/// original byte arrays to emit.
/// - "Consumer Device" generally has all device type controls like media, backlight, power
/// - "Keyboard Device" is a full featured keyboard including special keys like F13-F24
///
/// # Some example uses:
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x68, 0, 0, 0, 0]); // F13, Config/Control Panel`
///
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x70, 0, 0, 0, 0]); // F21, Touchpad toggle, XF86/Gnome`
///
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x71, 0, 0, 0, 0]); // F22, Touchpad on, XF86/Gnome`
///
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x72, 0, 0, 0, 0]); // F23, Touchpad off, XF86/Gnome`
///
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x82, 0, 0, 0, 0]); // Sleep`
///
/// `rogcore.virt_keys().press([0x01, 0, 0, 0x66, 0, 0, 0, 0]); // Power (menu)`
pub struct VirtKeys {
    device: UHIDDevice<std::fs::File>,
}

impl VirtKeys {
    pub fn new() -> Self {
        VirtKeys {
            device: UHIDDevice::create(CreateParams {
                name: String::from("Virtual ROG buttons"),
                phys: String::from(""),
                uniq: String::from(""),
                bus: Bus::USB,
                vendor: 0x0b05,
                product: 0x1866,
                version: 0,
                country: 0,
                // This is a device which emits the usage code as a whole, rather than as bits
                rd_data: [
                    // Consumer Device TLC
                    0x05, 0x0C, // Usage Page (Consumer)
                    0x09, 0x01, // Usage (Consumer Control)
                    0xA1, 0x01, // Collection (Application)
                    0x85, 0x02, //   Report ID (2)
                    0x19, 0x00, //   Usage Minimum (Unassigned)
                    0x2A, 0x3C, 0x02, //   Usage Maximum (AC Format)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0x3C, 0x02, //   Logical Maximum (572)
                    0x75, 0x10, //   Report Size (16)
                    0x95, 0x02, //   Report Count (2)
                    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State)
                    0xC0, //
                    0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
                    0x09, 0x06, // Usage (Keyboard)
                    0xA1, 0x01, // Collection (Application)
                    0x85, 0x01, //   Report ID (1)
                    0x75, 0x01, //   Report Size (1)
                    0x95, 0x08, //   Report Count (8)
                    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
                    0x19, 0xE0, //   Usage Minimum (0xE0)
                    0x29, 0xE7, //   Usage Maximum (0xE7)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x25, 0x01, //   Logical Maximum (1)
                    0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State)
                    0x95, 0x01, //   Report Count (1)
                    0x75, 0x08, //   Report Size (8)
                    0x81, 0x03, //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State)
                    0x95, 0x05, //   Report Count (5)
                    0x75, 0x01, //   Report Size (1)
                    0x05, 0x08, //   Usage Page (LEDs)
                    0x19, 0x01, //   Usage Minimum (Num Lock)
                    0x29, 0x05, //   Usage Maximum (Kana)
                    0x91, 0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State)
                    0x95, 0x01, //   Report Count (1)
                    0x75, 0x03, //   Report Size (3)
                    0x91, 0x03, //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State)
                    0x95, 0x1E, //   Report Count (30)
                    0x75, 0x08, //   Report Size (8)
                    0x15, 0x00, //   Logical Minimum (0)
                    0x26, 0xFF, 0x00, //   Logical Maximum (255)
                    0x05, 0x07, //   Usage Page (Kbrd/Keypad)
                    0x19, 0x00, //   Usage Minimum (0x00)
                    0x2A, 0xFF, 0x00, //   Usage Maximum (0xFF)
                    0x81, 0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State)
                    0xC0, // End Collection
                ]
                .to_vec(),
            })
            .map_err(|err| error!("Could not create virtual device: {:?}", err))
            .expect("Could not create virtual device"),
        }
    }

    /// A single on/off key press
    pub fn press(&mut self, input: [u8; 32]) {
        self.device.write(&input).unwrap();
        let mut reset = [0u8; 32];
        reset[0] = input[0];
        self.device.write(&reset).unwrap();
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum ConsumerKeys {
    Power = 0x30,
    Sleep = 0x32,
    Menu = 0x0040,

    MediaRecord = 0xB2,
    MediaFastFwd = 0xB3,
    MediaRewind = 0xB4,
    MediaNext = 0xB5,
    MediaPrev = 0xB6,
    MediaStop = 0xB7,
    MediaPlayPause = 0xCD,
    MediaPause = 0xB0,

    MediaVolMute = 0xE2,
    MediaVolUp = 0xE9,
    MediaVolDown = 0xEA,

    BacklightInc = 0x006F,
    BacklightDec = 0x0070,

    BacklightTog = 0x072, // USAGE (Backlight toggle? display toggle?)
    BacklightMin = 0x73,
    BacklightMax = 0x74,

    ControlConfig = 0x183,

    LaunchWordEditor = 0x184,
    LaunchTextEditor = 0x185,
    LaunchSpreadSheet = 0x186,
    LaunchGraphicsEditor = 0x187,
    LaunchPresentationApp = 0x188,
    LaunchDatabaseEditor = 0x189,
    LaunchEmailApp = 0x18A,
    LaunchNewsReader = 0x18B,
    LaunchCalendarApp = 0x018e,
    LaunchTaskManagementApp = 0x18F,
    LaunchWebBrowser = 0x196,
    ControlPanel = 0x19F,

    VideoOutStep = 0x82,

    Documents = 0x1A7,
    FileBrowser = 0x1B4,
    ImageBrowser = 0x1B6,
    AudioBrowser = 0x1B7,
    MovieBrowser = 0x1B8,
}

impl From<ConsumerKeys> for [u8; 32] {
    fn from(key: ConsumerKeys) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0] = 0x02; // report ID for consumer
        bytes[1] = key as u8;
        bytes[2] = (key as u16 >> 8) as u8;
        bytes
    }
}
