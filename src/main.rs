extern crate ctrlc;
extern crate joydev;
extern crate robust_arduino_serial;
extern crate serial;

use robust_arduino_serial::*;
use serial::prelude::*;
use std::time::Duration;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use joydev::{
    event_codes::AbsoluteAxis, event_codes::Key, Device, DeviceEvent, Error, GenericEvent,
};

// joydev repo: https://gitlab.com/gm666q/joydev-rs
// robust_arduino_serial example: https://github.com/araffin/rust-arduino-serial/blob/master/examples/arduino_serial.rs

// how to run: 1. connect dualshock4 to raspberry
//             2. sudo ds4drv --hidraw &
//             3. sudo ./dualshock

// Default settings of Arduino
// see: https://www.arduino.cc/en/Serial/Begin
const PORT_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() -> Result<(), Error> {
    // open serial port >>
    let serial_port = "/dev/ttyACM0";
    println!("Opening port: {:?}", serial_port);
    let mut port = serial::open(serial_port).unwrap();
    port.configure(&PORT_SETTINGS).unwrap();
    // timeout of 30s
    port.set_timeout(Duration::from_secs(30)).unwrap();

    let running = Arc::new(AtomicBool::new(true));

    // setup CTRL+C intrerrupt
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // open joistick controller
    let device = Device::open("/dev/input/js0")?;
    println!("{:#?}", device);

    while running.load(Ordering::SeqCst) {
        'inner: loop {
            let event = match device.get_event() {
                Err(error) => match error {
                    Error::QueueEmpty => break 'inner,
                    _ => panic!(
                        "{}: {:?}",
                        "called `Result::unwrap()` on an `Err` value", &error
                    ),
                },
                Ok(event) => event,
            };
            match event {
                DeviceEvent::Axis(ref event) => {
                    match event.axis() {
                        AbsoluteAxis::Hat0X => {
                            if event.value() < 10 {
                                write_i8(&mut port, 'a' as i8).unwrap();
                            }
                        }
                        _ => {
                            // ignore
                        }
                    }
                    println!("{:?}", event);
                }
                DeviceEvent::Button(ref event) => {
                    // see: https://gitlab.com/gm666q/joydev-rs/-/blob/master/joydev/src/event_codes/key.rs
                    println!("{:?}", event);
                    match event.button() {
                        Key::ButtonNorth => {
                            write_i8(&mut port, 'f' as i8).unwrap();
                        }
                        Key::ButtonSouth => {
                            write_i8(&mut port, 's' as i8).unwrap();
                        }
                        _ => {
                            // ignore
                        }
                    }
                }
            }
        }
        //println!("Queue empty");
    }

    Ok(())
}
