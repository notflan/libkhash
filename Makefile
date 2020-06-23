INSTALL:= /usr/local/lib
INSTALL-BIN:= /usr/local/bin
CLI:= cli

khash:
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	strip ./target/release/libkhash.so
	cd $(CLI) && $(MAKE) kana-hash

test:
	cargo test
	cd $(CLI) && $(MAKE)

install:
	cp -f ./target/release/libkhash.so $(INSTALL)/libkhash.so
	cp -f $(CLI)/build/kana-hash $(INSTALL-BIN)/kana-hash

uninstall:
	rm -f $(INSTALL)/libkana_hash.so
	rm -f $(INSTALL-BIN)/kana-hash
