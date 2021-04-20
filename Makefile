debug:
	cargo run

build:
	cargo build --release

run:
	./target/release/fare_estimation

test:
	cargo test

fmt:
	cargo fmt

bench-med:
	cargo bench -- medium_file

bench:
	cargo bench
