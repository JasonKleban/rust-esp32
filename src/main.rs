#![feature(restricted_std)]

use esp32_hal::{
    prelude::*,
    target,
    delay::Delay,
    i2c,
    dport::Split
};
use core::fmt::Write;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[export_name = "app_main"]
fn main() {
    let peripherals = target::Peripherals::take().unwrap();
    let pins = peripherals.GPIO.split();

    // peripherals.RTCCNTL.

    // disable_rtc_wdt();

    // let mut oled = pins.gpio5.into_open_drain_output();
    // let mut reset = pins.gpio16.into_open_drain_output();
    // let i2c = peripherals.I2C0;
    // let sda = pins.gpio4;
    // let scl = pins.gpio15;

    let mut delay = Delay::new();

    // let (mut dport, dport_clock_control) = peripherals.DPORT.split();

    // let interface = I2CDisplayInterface::new(i2c::I2C::new(
    //     i2c,
    //     i2c::Pins { sda, scl },
    //     400_000_u32,
    //     &mut dport
    // ));

    // oled.set_high().unwrap();
    // reset.set_low().unwrap();
    // reset.set_high().unwrap();

    // let mut display = Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate0,
    // ).into_terminal_mode();

    // display.init().unwrap();

    // display.clear().expect("BOOM!");

    // display.write_str("Hello World!").expect("BOOM!");

    // Blinky:

    let mut led = pins.gpio25.into_open_drain_output();

    loop {

        println!("Cycle");

        // display.clear().expect("BOOM!");
        // display.write_str("Hello World..").expect("BOOM!");

        led.set_high().unwrap();
        delay.delay_ms(1000_u32);

        // display.clear().expect("BOOM!");
        // display.write_str("Hello World...").expect("BOOM!");

        led.set_low().unwrap();
        delay.delay_ms(1000_u32);
    }
}

// unused
fn disable_rtc_wdt(rtccntl: &mut esp32::RTCCNTL) {
    /* Disables write protection */
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    /* Disables all wdt stages & the global watchdog flag itself */
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w
        .wdt_stg0()
        .bits(0x0)
        .wdt_stg1()
        .bits(0x0)
        .wdt_stg2()
        .bits(0x0)
        .wdt_stg3()
        .bits(0x0)
        .wdt_flashboot_mod_en()
        .clear_bit()
        .wdt_en()
        .clear_bit()
    });
    /* Re-enables write protection */
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}