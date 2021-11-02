build:
	cargo build --target x86_64-unknown-linux-musl

build-release:
	cargo build --target x86_64-unknown-linux-musl --release
