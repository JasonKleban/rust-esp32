
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

```
Serial port: COM3
Connecting...

Chip type:         ESP32 (revision 1)
Crystal frequency: 26MHz
Flash size:        8MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       7c:9e:bd:5b:9a:7c
   Compiling rust-esp32 v0.1.0 (C:\Users\Jason\Repos\rust-esp32)
    Finished release [optimized + debuginfo] target(s) in 6.71s
warning: unused import: `esp_idf_hal::prelude::*`
...
warning: unused import: `esp_idf_svc::sysloop::*`
...
warning: 2 warnings emitted

[00:00:01] ########################################      16/16      segment 0x1000
[00:00:00] ########################################       1/1       segment 0x8000
[00:00:11] ########################################     108/108     segment 0x10000

Flashing has completed!
```

7) Debugging

Umm... Now what?  Presumably it runs when Reset?  But how to tell without blindly succeeding in writing more code?  How do I configure/attach the VSCode debugger stuff to see, for starters, the console output?