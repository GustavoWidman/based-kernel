.PHONY: all kernel iso run clean

TARGET := x86_64-unknown-none
PROFILE := release
ISODIR = ./target/$(TARGET)/$(PROFILE)/iso
ISOPATH = ./target/$(TARGET)/$(PROFILE)/kernel.iso
BINPATH = ./target/$(TARGET)/$(PROFILE)/kernel

all: iso

kernel:
	cargo build --profile $(PROFILE) --target $(TARGET)

iso: kernel
	@rm -rf $(ISODIR)
	@rm -f $(ISOPATH)
	@mkdir -p $(ISODIR)/boot/grub
	@cp ./grub/grub.cfg $(ISODIR)/boot/grub/grub.cfg
	@cp $(BINPATH) $(ISODIR)/boot/kernel.bin
	@grub-mkrescue -o $(ISOPATH) $(ISODIR)

run: iso
	@qemu-system-x86_64 -cdrom $(ISOPATH) -m 512M -boot d -display curses

clean:
	cargo clean
	rm -rf target
