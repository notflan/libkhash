INSTALL:= /usr/local/lib

khash:
	cargo build --release
	strip ./target/release/libkhash.so

test:
	cargo test
	cd test && $(MAKE)

install:
	cp -f ./target/release/libkhash.so $(INSTALL)/libkhash.so

uninstall:
	rm -f $(INSTALL)/libkana_hash.so
