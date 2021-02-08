extern crate joydev;

use joydev::Device;

fn main() {
    // You should probably check what devices are available
    // by reading /dev/input directory or using udev.
    if let Ok(device) = Device::open("/dev/input/js0") {
        // Get an event and print it.
        println!("{:?}", device.get_event());
    }
}
