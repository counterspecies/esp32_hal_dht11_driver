
## esp32_hal-dht11-driver

esp32_hal-dht11-driver is is a Rust crate that reads temperature and humidity data from the DHT11 sensors for esp32 series. Forked from https://github.com/nor236/esp32-dht11-rs, and updated to use the esp32_hal Delay stuct instead of a generic struct which implements Delay fron the embedded_hal crate. This resolves some dependancy issues with certain esp32 crates.

This library is #![no_std] and depends  on embedded_hal and esp-hal.

## Usage
 
 
```rust
    let delay = Delay::new();
    let mut dht11 = DHT11::new(peripherals.GPIO2, delay);

    loop { 
        match dht11.read() {
            Ok(m) => println!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                m.temperature,
                m.humidity
            ),
            Err(error) => println!("An error occurred while trying to read sensor: {:?}", error),
        }
        delay.delay_millis(500);
    }

 ```

