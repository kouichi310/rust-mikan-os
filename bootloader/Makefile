build:
	cargo build --release

run: build target/x86_64-unknown-uefi/release/rust_mikan_os.efi
	../../mikanos-build-rust/devenv/run_qemu.sh target/x86_64-unknown-uefi/release/rust_mikan_os_bootloader.efi
