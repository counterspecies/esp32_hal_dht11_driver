
## esp32_hal-dht11-driver

This Rust library is a driver for DHT11 temperature and humidity sensor for esp32 series. Forked from https://github.com/nor236/esp32-dht11-rs, and updated to remove depancency from the embedded_hal crate, resolving some dependancy issues with certain esp32 crates. 


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


![dht11](image/dht11_temperature_humidity_sensor.jpg)
