mkimage -A arm -O u-boot -T firmware -d out/smol.bin -C none -a 0x42000000 out/smol.img
dd if=/dev/zero of=smol.fs bs=1M count=60
mkfs.vfat -F 32 smol.fs
dd if=/dev/zero of=out/smol bs=1M count=4
cat smol.fs >> out/smol
rm -f smol.fs
parted -s out/smol mklabel msdos
parted -s out/smol mkpart primary fat32 4 67
dd if=build/sunxi-spl.bin of=out/smol conv=notrunc bs=1024 seek=8
dd if=out/smol.img of=out/smol conv=notrunc bs=1024 seek=40
