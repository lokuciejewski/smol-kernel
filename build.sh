#!/bin/bash

echo "Starting build..."
if xargo build -Z build-std --release --target armv7a-none-eabihf 
then
    echo "Linking..."
    cd build
    if arm-none-eabi-gcc -T linker.ld -o out/myos.elf -ffreestanding -O2 -nostdlib boot.o ../target/armv7a-none-eabihf/release/libkernel.rlib ../target/armv7a-none-eabihf/release/deps/*.rlib
    then
        # arm-none-eabi-objcopy -O binary --remove-section .uncached out/myos.elf out/myos.bin  
        # mkimage -A arm -O u-boot -T script -C none -a 0 -e 0 -n 'Execute file.bin' -d build/bootscript out/boot.scr 
        echo "Mounting..."
        sudo mount /dev/mmcblk0p1 /media/usb/
        echo "Copying files..."
        sudo cp out/* /media/usb/
        echo "Unmounting..."
        sudo umount /media/usb/
        cd ..
        echo "Finished!"
    fi
fi