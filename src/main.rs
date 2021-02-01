extern crate dualshock4;
extern crate hidapi;

use hidapi::HidApi;

fn main() {
    let api = HidApi::new().expect("Failed to create HID API instance.");

    //println!("{:?}", api.devices().len());

    let controller = dualshock4::get_device(&api).expect("Failed to open device");

    loop {
        let data = dualshock4::read(&controller).expect("Failed to read data");

        println!("{:?}", data.buttons.dpad_left);
    }
}
