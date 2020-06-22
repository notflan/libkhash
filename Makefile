

khash:
	cargo build --release

install:
	cp ./target/release/libkana_hash.so /usr/lib/libkana_hash.so
