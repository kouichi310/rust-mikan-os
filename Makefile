build:
	cargo build --release

clean:
	cargo clean

run: build
	../mikanos-build-rust/devenv/run_qemu.sh target/x86_64-unknown-uefi/release/rust_mikan_os.efi