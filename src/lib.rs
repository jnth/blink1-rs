use hidapi::HidApi;
pub use hidapi::HidError;
use hidapi::HidResult;
pub use palette::rgb::Rgb;
pub use palette::Srgb;
use std::time::Duration;

const BLINK1_VENDOR_ID: u16 = 0x27b8;
const BLINK1_PRODUCT_ID: u16 = 0x01ed;

const BLINK1_REPORT_ID: u8 = 1;
const BLINK1_REPORT_SIZE: usize = 8;
const BLINK1_BUF_SIZE: usize = BLINK1_REPORT_SIZE + 1;

pub struct Blink1Device {
    device: hidapi::HidDevice,
}

impl Blink1Device {
    pub fn open_first() -> Result<Blink1Device, HidError> {
        match HidApi::new() {
            Ok(api) => {
                // Connect to device using its VID and PID
                match api.open(BLINK1_VENDOR_ID, BLINK1_PRODUCT_ID) {
                    Ok(device) => Ok(Blink1Device { device }),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn open(serial: &str) -> Result<Blink1Device, HidError> {
        match HidApi::new() {
            Ok(api) => {
                // Connect to device using its VID and PID
                match api.open_serial(BLINK1_VENDOR_ID, BLINK1_PRODUCT_ID, serial) {
                    Ok(device) => Ok(Blink1Device { device }),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_serials() -> Result<Vec<String>, HidError> {
        match HidApi::new() {
            Ok(api) => {
                let mut vec: Vec<String> = Vec::new();

                for device in api.device_list() {
                    if device.product_id() == BLINK1_PRODUCT_ID
                        && device.vendor_id() == BLINK1_VENDOR_ID
                    {
                        if let Some(serial) = &device.serial_number() {
                            vec.push(serial.to_string());
                        }
                    }
                }
                Ok(vec)
            }
            Err(e) => Err(e),
        }
    }

    pub fn off(&self, index: u8) -> HidResult<()> {
        self.fade_off(index, Duration::from_secs(0))
    }

    pub fn fade_off(&self, index: u8, fade_duration: Duration) -> HidResult<()> {
        self.fade_to_rgb(index, Rgb::new(0., 0., 0.), fade_duration)
    }

    pub fn rgb(&self, index: u8, color: Rgb) -> HidResult<()> {
        self.fade_to_rgb(index, color, Duration::from_secs(0))
    }

    pub fn fade_to_rgb(&self, index: u8, color: Rgb, fade_duration: Duration) -> HidResult<()> {
        let color: Srgb<u8> = color.into();
        let th = ((fade_duration.as_millis() as f32 / 10.).round() as u32) >> 8;
        let tl = ((fade_duration.as_millis() as f32 / 10.).round() as u32) & 0xff;

        let mut buf: [u8; BLINK1_BUF_SIZE] = [0; BLINK1_BUF_SIZE];

        buf[0] = BLINK1_REPORT_ID;
        buf[1] = b'c'; // command code for 'fade to rgb'
        buf[2] = color.red;
        buf[3] = color.green;
        buf[4] = color.blue;
        buf[5] = th as u8;
        buf[6] = tl as u8;
        buf[7] = index;

        self.device.send_feature_report(&buf)
    }
}
