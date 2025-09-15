#!/usr/bin/env nu

def main [kernel_bin: string] {
    let iso_dir = $"($kernel_bin | path dirname)/iso"

    rm -rf $iso_dir
    mkdir $"($iso_dir)/boot/grub"

    cp "grub/grub.cfg" $"($iso_dir)/boot/grub/grub.cfg"
    cp $kernel_bin $"($iso_dir)/boot/kernel.bin"

    grub-mkrescue -o $"($kernel_bin | path dirname)/kernel.iso" $iso_dir

    qemu-system-x86_64 -machine q35 -device virtio-net-pci,netdev=net0 -netdev user,id=net0,hostfwd=tcp::5555-:5555 -cdrom $"($kernel_bin | path dirname)/kernel.iso" -m 512M -boot d -display curses
}
