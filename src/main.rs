extern crate rusb;

use rusb::{DeviceHandle, Error};
use std::time::Duration;

static LED_MSG_LEN: usize = 17;
static LED_SPEED_BYTES: [u8; 3] = [0xe1, 0xeb, 0xf5];
static LED_INIT1: [u8; 2] = [0x5d, 0xb9];
static LED_INIT2: &'static str = "]ASUS Tech.Inc."; // 0x5d ASUS Tech.Inc.
static LED_INIT3: [u8; 6] = [0x5d, 0x05, 0x20, 0x31, 0x00, 0x08];
static LED_INIT4: &'static str = "^ASUS Tech.Inc."; // 0x5e ASUS Tech.Inc.
static LED_INIT5: [u8; 6] = [0x5e, 0x05, 0x20, 0x31, 0x00, 0x08];
static LED_SET: [u8; 2] = [0x5d, 0xb5];
static LED_APPLY: [u8; 2] = [0x5d, 0xb4];
static LED_BRIGHT_BYTES: [u8; 6] = [0x5a, 0xba, 0xc5, 0xc4, 0, 0]; // Pos 4 = set brightness

fn get_device(vendor: u16, product: u16) -> Result<DeviceHandle<rusb::GlobalContext>, Error> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == vendor && device_desc.product_id() == product {
            return device.open();
        }
    }
    Err(Error::NoDevice)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handle = get_device(0x0B05, 0x1866)?;
    handle.set_active_configuration(0).unwrap_or(());

    let config = handle.device().config_descriptor(0).unwrap();
    // Interface with outputs
    let mut keyboard_iface_num = 0;
    let mut led_iface_num = 0;
    for iface in config.interfaces() {
        for desc in iface.descriptors() {
            for endpoint in desc.endpoint_descriptors() {
                if endpoint.address() == 0x83 {
                    keyboard_iface_num = desc.interface_number();
                } else if endpoint.address() == 0x81 {
                    led_iface_num = desc.interface_number();
                }
            }
        }
    }

    if handle.kernel_driver_active(keyboard_iface_num).unwrap() {
        handle.detach_kernel_driver(keyboard_iface_num).unwrap();
    }
    if handle.kernel_driver_active(led_iface_num).unwrap() {
        handle.detach_kernel_driver(led_iface_num).unwrap();
    }

    let iface_out = config
        .interfaces()
        .nth(keyboard_iface_num as usize)
        .unwrap();
    let iface_led = config.interfaces().nth(led_iface_num as usize).unwrap();

    handle.claim_interface(iface_out.number()).unwrap();
    handle.claim_interface(iface_led.number()).unwrap();

    // WRITE LED
    // bmRequestType = 0x21
    // bRequest = 0x09
    // wValue = 0x035D
    // wIndex = 0
    println!("BEGIN INIT");
    handle.write_control(0x21, 0x09, 0x035D, 0, &LED_INIT1, Duration::new(0, 5))?;
    handle.write_control(
        0x21,
        0x09,
        0x035D,
        0,
        &LED_INIT2.as_bytes(),
        Duration::new(0, 5),
    )?;
    handle.write_control(0x21, 0x09, 0x035D, 0, &LED_INIT3, Duration::new(0, 5))?;
    handle.write_control(
        0x21,
        0x09,
        0x035D,
        0,
        &LED_INIT4.as_bytes(),
        Duration::new(0, 5),
    )?;
    handle.write_control(0x21, 0x09, 0x035D, 0, &LED_INIT5, Duration::new(0, 5))?;
    println!("END INIT");

    println!("BEGIN BRIGHT");
    let mut bright = LED_BRIGHT_BYTES.clone();
    bright[4] = 1;
    println!("{:x?}", bright);
    handle.write_control(0x21, 0x09, 0x035D, 0, &bright, Duration::new(0, 5))?;
    println!("END BRIGHT");
    handle.write_control(0x21, 0x09, 0x035D, 0, &LED_SET, Duration::new(0, 5))?;
    handle.write_control(0x21, 0x09, 0x035D, 0, &LED_APPLY, Duration::new(0, 5))?;
    println!("END");

    // READ KEYBOARD
    let mut buf = [0; 32];
    loop {
        match handle.read_interrupt(0x83, &mut buf, Duration::new(0, 5)) {
            Ok(_) => println!("{:?}", buf),
            Err(err) => match err {
                Error::Timeout => {}
                _ => return Err(Box::new(err)),
            },
        }
    }
}
