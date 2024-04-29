kernel:
	cargo build

iso: kernel
	@mkdir -p build/
	@cp -r image build/isofiles
	@cp target/x86_64/debug/basic-os-rust build/isofiles/boot/kernel.bin
	@grub-mkrescue -o build/os.iso build/isofiles 2> /dev/null

run: iso
	@qemu-system-x86_64 -cdrom build/os.iso

clean:
	cargo clean
	rm -r build