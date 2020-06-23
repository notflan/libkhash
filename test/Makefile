SRC:= src/*.c
INCLUDE:=include/
LIB:=../target/release
BUILD:=build/
CFLAGS:= -g -Wall -pedantic
LFLAGS:= -L$(LIB) -lkhash


all: clean test

clean:

test:
	gcc $(SRC) -I$(INCLUDE) $(CFLAGS) -o $(BUILD)/$@ $(LFLAGS)
	$(BUILD)/$@
