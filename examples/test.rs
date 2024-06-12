extern crate blink1;

use blink1::Blink1Device;
use palette::rgb::Rgb;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    let sleep = Duration::from_secs(10);

    match Blink1Device::get_serials() {
        Ok(serials) => {
            for serial in serials {
                println!("Serial number: {}", serial)
            }
        }
        Err(e) => println!("{}", e),
    }

    let device = Blink1Device::open_first().expect("Failed to open device.");

    println!("Fade to red...");
    device
        .fade_to_rgb(
            0,
            Rgb::from_str("#ff0000").unwrap().into(),
            Duration::from_secs(1),
        )
        .expect("Failed to set color.");

    thread::sleep(sleep);

    println!("Turn off...");
    device.off(0).expect("Failed to set color.");
}
