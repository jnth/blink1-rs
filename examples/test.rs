extern crate blink1;

use blink1::Blink1Device;
use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(1000);

    match Blink1Device::get_serials() {
        Ok(serials) => {
            for serial in serials {
                println!("Serial number: {}", serial)
            }
        }
        Err(e) => println!("{}", e)
    }

    let device = Blink1Device::open_first().expect("Failed to open device.");

    println!("Fade to red...");
    device
        .fade_to_rgb(0, 255, 0, 0)
        .expect("Failed to set color.");

    thread::sleep(ten_millis);

    println!("Turn off...");
    device
        .fade_off(0)
        .expect("Failed to set color.");
}
