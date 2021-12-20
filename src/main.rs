//use embedded_svc::anyerror::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::sysloop::*;
use std::{thread, time::Duration};

#[export_name = "app_main"]
fn main() -> ! {
    loop {
        println!("Hello world!");
        thread::sleep(Duration::from_secs(1));
    }
}