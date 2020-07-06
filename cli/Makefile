SRC:= src/*.c
INCLUDE:=include/
LIB:=lib
BUILD:=build
CFLAGS:= -g -Wall -pedantic
LFLAGS:= -lkhash


all: clean test kana-hash

clean:
	rm -f $(BUILD)/*

kana-hash:
	gcc $(SRC) -I$(INCLUDE) $(CFLAGS) -o $(BUILD)/$@ $(LFLAGS)
	$(BUILD)/$@ test

test:
	gcc $(SRC) -I$(INCLUDE) -DTEST $(CFLAGS) -o $(BUILD)/$@ $(LFLAGS)
	$(BUILD)/$@


