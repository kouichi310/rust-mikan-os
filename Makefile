all: bootloader kernel

.PHONY: kernel
kernel:
	${MAKE} -C $@ build

.PHONY: bootloader
bootloader:
	${MAKE} -C bootloader build

run: all
	../mikanos-build-rust/devenv/run_qemu.sh target/x86_64-unknown-uefi/release/rust_mikan_os_bootloader.efi target/x86_64-rust-mikan-os-elf/release/rust_mikan_os_kernel

copy_memmap:
	hdiutil attach disk.img  && cp '/Volumes/MIKAN OS/memmap.csv' . && hdiutil detach disk4
