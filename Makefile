pre-install-ubuntu:
	apt install llvm-dev libclang-dev clang cmake libusb-dev
pre-install-mac:
	brew install llvm cmake libusb
install:
	cargo build
codegen:
	cargo run --example codegen
start:
	cargo run
