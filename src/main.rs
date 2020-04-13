extern crate rusb;

use rusb::Direction;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = rusb::Context::new().unwrap();

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x0B05 && device_desc.product_id() == 0x1866 {
            let mut handle = device.open()?;
            handle.set_active_configuration(1).unwrap_or(());

            let config = device.config_descriptor(0).unwrap();
            // Interface with outputs
            let mut keyboard_out = 0;
            let mut led_config = 0;
            for iface in config.interfaces() {
                for desc in iface.descriptors() {
                    for endpoint in desc.endpoint_descriptors() {
                        if endpoint.direction() == Direction::In && endpoint.address() == 0x83 {
                            keyboard_out = desc.interface_number();
                        } else if endpoint.direction() == Direction::Out
                            && endpoint.address() == 0x81
                        {
                            led_config = desc.interface_number();
                        }
                    }
                }
            }
            let iface_out = config.interfaces().nth(keyboard_out as usize).unwrap();
            let iface_led = config.interfaces().nth(led_config as usize).unwrap();

            if handle.kernel_driver_active(0).unwrap() {
                handle.detach_kernel_driver(0).unwrap();
            }

            handle.claim_interface(iface_out.number()).unwrap();
            handle.claim_interface(iface_led.number()).unwrap();

            // WRITE LED

            // READ KEYBOARD
            let mut buf = [0; 32];
            loop {
                match handle.read_interrupt(0x83, &mut buf, Duration::new(5, 0)) {
                    Ok(_) => println!("{:?}", buf),
                    Err(err) => {
                        // RELEASE
                        // not needed handle.release_interface(interface.number())?;
                        // handle.attach_kernel_driver(interface.number())?;
                        return Err(Box::new(err));
                    }
                }
            }
        }
    }
    Ok(())
}
