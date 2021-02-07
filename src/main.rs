use btleplug::api::{Central, CentralEvent};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::ConnectedAdapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
use std::str::FromStr;

// adapter retrieval works differently depending on your platform right now.
// API needs to be aligned.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().unwrap();
    if adapters.len() <= 0 {
        panic!("Bluetooth adapter(s) were NOT found, sorry...\n");
    }
    adapters.into_iter().nth(0).unwrap()
}

#[cfg(target_os = "linux")]
fn get_central(manager: &Manager) -> ConnectedAdapter {
    let adapters = manager.adapters().unwrap();
    if adapters.len() <= 0 {
        panic!("Bluetooth adapter(s) were NOT found, sorry...\n");
    }
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    adapter
        .connect()
        .expect("Error connecting to BLE Adapter....")
}

fn main() {
    let _peripheral_address = btleplug::api::BDAddr::from_str("B8:27:EB:11:87:85").unwrap();

    let manager = Manager::new().unwrap();

    // get the first bluetooth adapter
    // connect to the adapter
    let central = get_central(&manager);

    // start scanning for devices
    central
        .start_scan()
        .expect("Can't scan BLE adapter for connected devices...");
    // instead of waiting, you can use central.on_event to be notified of
    // new devices

    // Add ourselves to the central event handler output now, so we don't
    // have to carry around the Central object. We'll be using this in
    // connect anyways.
    let on_event = move |event: CentralEvent| match event {
        CentralEvent::DeviceDiscovered(bd_addr) => {
            println!("DeviceDiscovered: {:?}", bd_addr);
        }
        CentralEvent::DeviceConnected(bd_addr) => {
            println!("DeviceConnected: {:?}", bd_addr);
        }
        CentralEvent::DeviceDisconnected(bd_addr) => {
            println!("DeviceDisconnected: {:?}", bd_addr);
        }
        _ => {}
    };
    // bluetooth event handling
    central.on_event(Box::new(on_event));
    loop {}
}

// https://github.com/deviceplug/btleplug
// https://github.com/deviceplug/btleplug/blob/master/src/api/mod.rs
// https://book.async.rs/
// https://docs.rs/async-std/1.6.1/async_std/
