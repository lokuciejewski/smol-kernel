
# Smol Kernel for ARMv7

Developed and tested on NanoPi Neo v1.4

## Building and running

### System requirements

`cargo`, `xargo`, a gcc toolchain for armv7 (`arm-none-eabihf-gcc` or equivalent), a Rust toolchain for armv7 installed (`armv7a-none-eabihf` or equivalent)

### Running the kernel

Use the `./build.sh` script to automatically build all the necessary components. Then copy `boot.scr` and `smol.bin` to the root of the SD card.

### Building the executable

Run

`xargo build -Z build-std --release --target armv7a-none-eabihf`

 followed by

 `arm-none-eabi-gcc -T linker.ld -o out/smol.elf -ffreestanding -O2 -nostdlib boot.o ../target/armv7a-none-eabihf/release/libkernel.rlib ../target/armv7a-none-eabihf/release/deps/*.rlib`

  to build and link the executable.

Copy `smol.elf` file to the sd-card and use `u-boot` to load and run it.

### Running the executable

`U-boot` commands example on NanoPi Neo (may be the same for other boards):

`fatload mmc 0 0x42000000 smol.elf`

`go 0x42000000`

Then connect the device using serial UART console with 115200 8N1 (no hardware flow control!) settings.

## Currently implemented

- Basic register support (UART initialization)
- GPIO support
- RTC support
- Basic Heap Allocator using `linked-list-allocator`
