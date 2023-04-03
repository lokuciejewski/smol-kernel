mkimage -A arm -O u-boot -T firmware -d out/myos.bin -C none -a 0x42000000 out/myos.img
dd if=/dev/zero of=myos.fs bs=1M count=60
mkfs.vfat -F 32 myos.fs
dd if=/dev/zero of=out/myos bs=1M count=4
cat myos.fs >> out/myos
rm -f myos.fs
parted -s out/myos mklabel msdos
parted -s out/myos mkpart primary fat32 4 67
dd if=build/sunxi-spl.bin of=out/myos conv=notrunc bs=1024 seek=8
dd if=out/myos.img of=out/myos conv=notrunc bs=1024 seek=40
