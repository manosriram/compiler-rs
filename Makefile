run:
	cargo build && ./target/debug/comp

build:
	cargo build

test:
	cargo test

buildnowarn:
	RUSTFLAGS="-A warnings" cargo build

runnowarn:
	RUSTFLAGS="-A warnings" cargo run
