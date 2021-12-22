use esp_idf_hal::delay;
use esp_idf_hal::prelude::*;
use esp_idf_hal::i2c;
use core::fmt::Write;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[export_name = "app_main"]
fn main() {

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut oled = pins.gpio5.into_output_od().unwrap();
    let mut reset = pins.gpio16.into_output_od().unwrap();
    let i2c = peripherals.i2c0;
    let sda = pins.gpio4;
    let scl = pins.gpio15;

    let config = <i2c::config::MasterConfig as Default>::default().baudrate(400.kHz().into());

    let interface = I2CDisplayInterface::new(i2c::Master::<i2c::I2C0, _, _>::new(
        i2c,
        i2c::MasterPins { sda, scl },
        config,
    ).unwrap());

    oled.set_high().unwrap();
    reset.set_low().unwrap();
    reset.set_high().unwrap();

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();

    display.init().unwrap();

    display.clear().expect("BOOM!");

    display.write_str("Hello World!").expect("BOOM!");

    // Blinky:

    let mut led = pins.gpio25.into_output_od().unwrap();

    let mut delay = delay::FreeRtos;

    loop {

        println!("Cycle");

        display.clear().expect("BOOM!");
        display.write_str("Hello World..").expect("BOOM!");

        led.set_high().unwrap();
        delay.delay_ms(1000_u32);

        display.clear().expect("BOOM!");
        display.write_str("Hello World...").expect("BOOM!");

        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}