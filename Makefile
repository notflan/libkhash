INSTALL:= /usr/local/lib
INSTALL-BIN:= /usr/local/bin
INSTALL-INCLUDE:=/usr/local/include
CLI:= cli


khash:
	RUSTFLAGS="-C target-cpu=native" cargo build --release
	strip ./target/release/libkhash.so
	cd $(CLI) && $(MAKE) kana-hash

khash-nonative:
	cargo build --release
	cd $(CLI) && $(MAKE) kana-hash

test:
	RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo test
	RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo bench
	cd $(CLI) && $(MAKE)

install:
	cp -f ./target/release/libkhash.so $(INSTALL)/libkhash.so
	cp -f ./target/release/libkhash.a $(INSTALL)/libkhash.a
	cp -f $(CLI)/build/kana-hash $(INSTALL-BIN)/kana-hash
	cp -f include/khash.h $(INSTALL-INCLUDE)/khash.h

uninstall:
	rm -f $(INSTALL)/libkhash.so
	rm -f $(INSTALL)/libkhash.a
	rm -f $(INSTALL-BIN)/kana-hash
	rm -f $(INSTALL-INCLUDE)/khash.h
