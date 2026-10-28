# motor_controller

ESP32-C3 motor controller: Rust firmware (ESP-IDF) and a KiCad board design.

## Layout

- `firmware_esp32c3/`: Rust firmware (crate `surco`)
- `doc/`: KiCad schematic, PCB, and PDF
- `firmware_esp32c3/doc/Guide.md`: system notes (German)

## Build and flash

Needs the ESP Rust toolchain (`espup`) and `cargo-espflash`.

```sh
cd firmware_esp32c3
make flash     # release build, then flash
make monitor   # flash and open the serial monitor
```

The current firmware ramps PWM duty from 1% to 99% at 5 kHz on GPIO5, then repeats.
