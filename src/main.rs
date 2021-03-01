extern crate joydev;
extern crate robust_arduino_serial;
extern crate serial;

use robust_arduino_serial::*;
use serial::prelude::*;
use std::time::Duration;

use std::thread;

use joydev::{event_codes::AbsoluteAxis, event_codes::Key, Device, DeviceEvent, GenericEvent};

use async_std::{channel, channel::TryRecvError, io, task};

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

enum Notification {
    ControllerButton(joydev::ButtonEvent),
    ControllerAxis(joydev::AxisEvent),
    SerialInput(u8),
    NetworkCommand(String),
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // open serial port >>
    let serial_port = "/dev/ttyACM0";
    println!("Opening port: {:?}", serial_port);
    let mut port = serial::open(serial_port)?;
    port.configure(&PORT_SETTINGS)?;
    // timeout of 30s
    port.set_timeout(Duration::from_secs(30))?;

    // open joistick controller
    let device = Device::open("/dev/input/js0")?;
    println!("{:#?}", device);

    // TODO: use std::sync::mpsc::channel (no need for async if the threads are spawned anyway)
    // see: https://www.youtube.com/watch?v=b4mS5UPHh20
    let (sender, receiver) = channel::unbounded::<Notification>();

    // Dualshock PS4 controller events
    {
        let dualshock_sender = sender.clone();
        thread::spawn(move || loop {
            // TODO: is it blocking? If not, it does not need a separate thread
            match device.get_event() {
                Err(error) => match error {
                    joydev::Error::QueueEmpty => (), // TODO: wait?
                    _ => panic!(
                        "{}: {:?}",
                        "called `Result::unwrap()` on an `Err` value", &error
                    ),
                },
                Ok(event) => {
                    match event {
                        DeviceEvent::Axis(event) => {
                            let s = dualshock_sender.clone();
                            task::spawn(async move {
                                s.send(Notification::ControllerAxis(event)).await.unwrap();
                            });
                        }
                        DeviceEvent::Button(event) => {
                            // see: https://gitlab.com/gm666q/joydev-rs/-/blob/master/joydev/src/event_codes/key.rs
                            let s = dualshock_sender.clone();
                            task::spawn(async move {
                                s.send(Notification::ControllerButton(event)).await.unwrap();
                            });
                        }
                    }
                }
            }
        });
    }

    // serial port reciever
    {
        let serial_port_sender = sender.clone();
        thread::spawn(move || loop {
            match read_i8(&mut port) {
                Ok(byte) => {
                    let s = serial_port_sender.clone();
                    task::spawn(async move {
                        s.send(Notification::SerialInput(byte as u8)).await.unwrap();
                    });
                }
                _ => panic!(),
            }
        });
    }

    // TODO: network reveiver
    {}

    // event processor
    {
        // TODO: use iterator on Reciever
        loop {
            let result = receiver.recv().await;
            //println!("Received: {:?}", result);
            if let Ok(notification) = result {
                match notification {
                    Notification::ControllerButton(button) => {
                        //write_i8(&mut port, 's' as i8);
                        // TODO: https://stackoverflow.com/questions/53440321/how-to-use-serial-port-in-multiple-threads-in-rust
                    }
                    Notification::ControllerAxis(axis) => {}
                    Notification::SerialInput(data) => {}
                    Notification::NetworkCommand(_data) => {
                        unimplemented!()
                    }
                }
            }
        }
    }

    unreachable!()
}
