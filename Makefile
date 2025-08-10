all:	clean debug release

debug:	test
	cargo build

release:	test
	cargo build --release

clean:
	cargo clean

test:
	cargo test

src/ptp.rs:	/usr/include/linux/ptp_clock.h
	bindgen $< -o $@ \
		--raw-line '#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]' \
		--rust-target "1.68" \
		--with-derive-default
