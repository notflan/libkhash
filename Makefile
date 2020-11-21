INSTALL:= /usr/lib64
INSTALL-BIN:= /usr/bin
INSTALL-INCLUDE:=/usr/include
CLI:= cli

PROJECT=khash

BUILD:=./target/release
DEBUG:=./target/debug

.PHONY: $(PROJECT)
$(PROJECT): release

.PHONY: release
release: $(BUILD)/lib$(PROJECT).so

.PHONY: debug
debug: $(DEBUG)/lib$(PROJECT).so

$(BUILD)/lib$(PROJECT).so: RUSTFLAGS?= -C target-cpu=native 
$(BUILD)/lib$(PROJECT).so:
	RUSTFLAGS="$(RUSTFLAGS)" cargo build --release
	strip $@
	cd $(CLI) && $(MAKE) release

$(DEBUG)/lib$(PROJECT).so:
	RUSTFLAGS="$(RUSTFLAGS)" cargo build
	cd $(CLI) && $(MAKE) debug

.PHONY: khash-nonative
khash-nonative:
	$(MAKE) RUSTFLAGS="" $(BUILD)/libkhash.so

.PHONY: test
test: RUSTFLAGS+= -C target-cpu=native -C opt-level=3
test: | clean
	RUSTFLAGS="$(RUSTFLAGS)" cargo test
	RUSTFLAGS="$(RUSTFLAGS)" cargo bench
	cd $(CLI) && $(MAKE) test

clean:
	rm -f $(BUILD)/lib
	cd $(CLI) && make clean

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
