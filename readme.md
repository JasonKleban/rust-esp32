
# Hello World on esp32

There's no reason the dev platform needs to matter, but I happen to be attempting this from Windows 10 x64 using VS Code and a basic Rust/cargo environment already working.

Thanks to https://kerkour.com/rust-on-esp32/ for the hard parts, really.

## Setup

The idealized path, as I believe it to be:

1) Clone `https://github.com/esp-rs/rust-build` locally and just run `./Install-RustToolchain.ps1` but ignore the `idf.py` commands since I don't have/want python and I think those are optional.  DO add the environment variables to powershell (and restart powershell session).

2) `cargo install cargo-espflash cargo-espmonitor`

3) Clone _this_ repo locally.

Note the vscode settings.json that tell rust-analyzer that `"RUSTUP_TOOLCHAIN": "esp"` which is akin to `"rustc +esp"`.  This helps rust-analyizer not crash, but I don't know that it is working ideally.

4) Plug in your ESP32 (ex. "HiLetgo ESP32 OLED WiFi Kit ESP-32 0.96 Inch Blue OLED Display WiFi+Bluetooth CP2012" $18) via typical USB connection.  Check Windows' Device Manager to figure out which COM port the device is attached to (ex. COM3).

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

7) `espmonitor --speed 115200 COM3`

```
ESPMonitor 0.6.0

Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

Opening COM3 with speed 115200
Resetting device... done
ets Jun  8 2016 00:22:57
rst:0x1 (POWERON_RESET),boot:0x17 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (133) cpu_start: Pro cpu up.
I (133) cpu_start: Starting app cpu, entry point is 0x400810d0
I (0) cpu_start: App cpu up.
I (147) cpu_start: Pro cpu start user code
I (147) cpu_start: cpu freq: 160000000
I (147) cpu_start: Application information:
I (152) cpu_start: Project name:     esp-idf
I (157) cpu_start: App version:      ac78a27-dirty
I (162) cpu_start: Compile time:     Dec 20 2021 12:59:47
I (168) cpu_start: ELF file SHA256:  0000000000000000...
I (174) cpu_start: ESP-IDF:          4.3.1
I (179) heap_init: Initializing. RAM available for dynamic allocation:
I (186) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (192) heap_init: At 3FFB3570 len 0002CA90 (178 KiB): DRAM
I (199) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (205) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (211) heap_init: At 4008C2F8 len 00013D08 (79 KiB): IRAM
I (219) spi_flash: detected chip: gd
I (222) spi_flash: flash io: dio
W (226) spi_flash: Detected size(8192k) larger than the size in the binary image header(4096k). Using the size in the binary image header.
I (240) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
Hello world!
Hello world!
Hello world!
Hello world!
...
```

8) Debugging

Not sure how to attach a debugger to this, but espmonitor seems like a great start.

## TODO

- [X] Onboard LED Blinky

## This branch

This branch is an attempt to get away from the IDF stuff that I don't understand.

On startup, this basically repeats.  I'm not sure how to interpret this output.  What does this mean?

```
Opening COM3 with speed 115200
Resetting device... done
epc3=0x00000000, excvaddr=0x00000000, depc=0x00000000
ets Jun  8 2016 00:22:57
rst:0x1 (POWERON_RESET),boot:0x17 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
Fatal exception (0): IllegalInstruction
epc1=0x400d0020, epc2=0x00000000, epc3=0x00000000, excvaddr=0x00000000, depc=0x00000000
```