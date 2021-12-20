use esp_idf_hal::delay;
use esp_idf_hal::prelude::*;

static mut CYCLES: u32 = 30;

#[export_name = "app_main"]
fn main() {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut delay = delay::FreeRtos;
    let mut led = pins.gpio25.into_output_od().unwrap();

    while get_cycles() > 0 {

        println!("Hello world!");

        led.set_high().unwrap();
        delay.delay_ms(1000_u32);

        println!("Hello world!");

        led.set_low().unwrap();
        delay.delay_ms(1000_u32);

        decr_cycles();
    }
}

fn get_cycles() -> u32 {
    unsafe { core::ptr::read_volatile(&CYCLES) }
}

fn decr_cycles() {
    unsafe {
        let cycles = core::ptr::read_volatile(&CYCLES);

        if cycles > 0 {
            core::ptr::write_volatile(&mut CYCLES, cycles - 1);
        }
    }
}