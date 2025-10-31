#![no_std]

use esp_hal::time::Instant;
use esp_hal::delay::Delay;
#[derive(Debug)]
pub enum SensorError {
    ChecksumMismatch,
    Timeout,
    PinError,
}

#[derive(Debug, Copy, Clone)]
pub struct Reading {
    pub humidity: u8,
    pub temperature: i8,
}

pub struct DHT11 {
    pub delay: Delay,
}

const _ERROR_CHECKSUM: u8 = 254; // Error code indicating checksum mismatch.
const ERROR_TIMEOUT: u8 = 253; // Error code indicating a timeout occurred during reading.
const TIMEOUT_DURATION: u64 = 1000; // Duration (in milliseconds) to wait before timing out.
impl DHT11 {
    pub fn new(delay: Delay) -> Self {
        Self { delay }
    }

    pub fn read(&mut self, pin: &mut esp_hal::gpio::Flex) -> Result<Reading, SensorError> {
        let data = self.read_raw(pin)?;
        let rh = data[0];
        let temp_signed = data[2];
        let temp = {
            let (signed, magnitude) = convert_signed(temp_signed);
            let temp_sign = if signed { -1 } else { 1 };
            temp_sign * magnitude as i8
        };

        Ok(Reading {
            temperature: temp,
            humidity: rh,
        })
    }

    fn read_raw(&mut self, pin: &mut esp_hal::gpio::Flex) -> Result<[u8; 5], SensorError> {
        pin.set_output_enable(true);
        pin.set_low();
        self.delay.delay_millis(20); 
        pin.set_high();
        self.delay.delay_micros(40);
        pin.set_input_enable(true);

        let now = Instant::now();

        while pin.is_high() {
            if now.elapsed().as_millis() > TIMEOUT_DURATION {
                // println!("wait for low timeout.");
                return Err(SensorError::Timeout);
            }
        }
 
        if pin.is_low() {
            self.delay.delay_micros(80);
            if pin.is_low() {
                return Err(SensorError::Timeout);
            }
        }
        self.delay.delay_micros(80);
        let mut buf = [0; 5];
        let tmp = 0..5;
        for idx in tmp {
            buf[idx] = self.read_byte(pin);
            if buf[idx] == ERROR_TIMEOUT {
                return Err(SensorError::Timeout);
            }
        }
        let sum = buf[0]
            .wrapping_add(buf[1])
            .wrapping_add(buf[2])
            .wrapping_add(buf[3]);

        if buf[4] == sum {
            Ok(buf)// Success
        } else {
            Err(SensorError::ChecksumMismatch)
        }
    }
    fn read_byte(&mut self, pin: &mut esp_hal::gpio::Flex) -> u8 {
        let mut buf = 0u8;
        for idx in 0..8u8 {
            while pin.is_low() {}
            self.delay.delay_micros(30); 
            if pin.is_high() {
                buf |= 1 << (7 - idx);
            }
            while pin.is_high() {}
        }
        buf
    }
}

fn convert_signed(signed: u8) -> (bool, u8) {
    let sign = signed & 0x80 != 0;
    let magnitude = signed & 0x7F;
    (sign, magnitude)
}
