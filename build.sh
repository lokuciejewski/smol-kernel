#!/bin/bash

echo "Starting build..."
if xargo build -Z build-std --release --target armv7a-none-eabihf 
then
    mkdir -p build/out/
    mkdir -p build/mnt/
    echo "Linking..."
    cd build
    if arm-none-eabi-gcc -T linker.ld -o out/smol.elf -ffreestanding -O2 -nostdlib boot.o ../target/armv7a-none-eabihf/release/libkernel.rlib ../target/armv7a-none-eabihf/release/deps/*.rlib
    then
        arm-none-eabi-objcopy -O binary --remove-section .uncached out/smol.elf out/smol.bin  
        mkimage -A arm -O u-boot -T script -C none -a 0 -e 0 -n 'Execute smol.bin' -d bootscript out/boot.scr 
        # echo "Mounting..."
        # sudo mount $1 mnt/
        # echo "Copying files..."
        # sudo cp out/* mnt/ 
        # echo "Unmounting..."
        # sudo umount mnt/
        cd ..
        echo "Finished!"
    fi
fi
