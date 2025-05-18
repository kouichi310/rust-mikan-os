build:
	cargo build --release

clean:
	cargo clean

run: build
	mikanos-build-rust/devenv/run_qemu.sh target/x86_64-rust-mikan-os-elf/release/rust_mikan_os