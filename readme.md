
# Hello World on esp32

There's no reason the dev platform needs to matter, but I happen to be attempting this from Windows 10 x64 using VS Code and a basic working Rust/cargo environment already working.

This project isn't currently successfully getting flashed to the chip (and is therefore in no way validated), but I feel like I'm getting close.

## Setup

The idealized path, as I believe it to be:

1) Clone `https://github.com/esp-rs/rust-build` locally and just run `./Install-RustToolchain.ps1` but ignore the `idf.py` commands since I don't have/want python and I think those are optional.  DO add the environment variables to powershell (and restart powershell session).

2) `cargo install cargo-espflash`

3) Clone _this_ repo locally.

4) Plug in your ESP32 (ex. "HiLetgo ESP32 OLED WiFi Kit ESP-32 0.96 Inch Blue OLED Display WiFi+Bluetooth CP2012" $18) via typical USB connection.  Check Windows' Device Manager to figure out which COM port the device is attached to (ex. COM3).  While the device has power, hold down PRG button during a press of RST - device should be ready to flash.

5) `cargo espflash board-info COM3`

```
Serial port: COM3
Connecting...

Chip type:         ESP32 (revision 1)
Crystal frequency: 26MHz
Flash size:        8MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       ...
```

6) `cargo +esp espflash --release COM3`

But after a bunch of compilation steps, I get six of these errors:

```
error[E0532]: expected tuple struct or tuple variant, found function `Ok`
   --> C:\Users\Jason\.cargo\registry\src\github.com-1ecc6299db9ec823\embuild-0.24.5\src\utils.rs:123:13
    |
123 |             Ok(result) => {
    |             ^^ not a tuple struct or tuple variant
    |
   ::: C:\Users\Jason\.cargo\registry\src\github.com-1ecc6299db9ec823\embuild-0.24.5\src\bindgen.rs:164:5
    |
164 |     cmd!("rustup", "run", "stable", "rustfmt", output_file)?;
    |     ------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `cmd` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these items instead
    |
1   | use serde::__private::Ok;
    |
1   | use std::result::Result::Ok;
    |
1   | use core::result::Result::Ok;
    |
```