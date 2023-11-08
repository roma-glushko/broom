lint:
	@cargo fmt
	@cargo clippy

run:
	@cargo run

build:
	@cargo build

build-release:
	@cargo build --release
