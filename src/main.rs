#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        // let peripherals = target::Peripherals::take().unwrap();
        // let pins = peripherals.GPIO.split();

        // let mut delay = Delay::new();

        // let mut led = pins.gpio25.into_open_drain_output();

        // loop {

        //     led.set_high().unwrap();
        //     delay.delay_ms(1000_u32);

        //     led.set_low().unwrap();
        //     delay.delay_ms(1000_u32);
        // }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}