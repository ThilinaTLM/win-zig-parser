.PHONY: all

all: install_rust build_project copy_bin

install_rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	. "${HOME}/.cargo/env"

build_project:
	cargo build --release

copy_bin:
	cp ./target/release/win-zig ./winzigc
