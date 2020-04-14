.PHONY

build:
	cargo build 

test: 
	RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo test