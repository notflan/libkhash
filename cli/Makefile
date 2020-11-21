SRC = $(wildcard src/*.c)
INCLUDE =include/

BUILD:=build

RELEASE_CFLAGS?=  -O3 -flto -fgraphite
RELEASE_LDFLAGS?= -O3 -flto

DEBUG_CFLAGS?=    -g -O0
DEBUG_LDFLAGS?=      -O0

LIB = lib

CFLAGS+= $(addprefix -I,$(INCLUDE)) -Wall -pedantic --std=gnu11 
LDFLAGS+= $(addprefix -L,$(LIB)) -lkhash

PROJECT = kana-hash
OBJ = $(addprefix ./obj/$(SRC:.c=.o))
OUTPUT = $(BUILD)/$(PROJECT)

.PHONY: release
release: | dirs $(OUTPUT)-release

.PHONY: debug
debug: | dirs $(OUTPUT)-debug

.PHONY: test
test: | dirs $(OUTPUT)-test

dirs:
	@mkdir -p obj/src
	@mkdir -p $(BUILD)
	@ln -sf ./$(PROJECT)-release $(OUTPUT)

obj/%.o: %.c
	$(CC) -c $< $(CFLAGS) -o $@ $(LDFLAGS)

$(OUTPUT)-debug: CFLAGS := $(CFLAGS) $(DEBUG_CFLAGS)
$(OUTPUT)-debug: LDFLAGS:= $(LDFLAGS) $(DEBUG_LDFLAGS)
$(OUTPUT)-debug: $(OBJ)
	$(CC) $^ $(CFLAGS) -o $@ $(LDFLAGS)

$(OUTPUT)-release: CFLAGS := $(CFLAGS) $(RELEASE_CFLAGS)
$(OUTPUT)-release: LDFLAGS:= $(LDFLAGS) $(RELEASE_LDFLAGS)
$(OUTPUT)-release: $(OBJ)
	$(CC) $^ $(CFLAGS) -o $@ $(LDFLAGS)
	strip $@

$(OUTPUT)-test: CFLAGS:= $(CFLAGS) -DTEST
$(OUTPUT)-test: $(OBJ)
	$(CC) $^ $(CFLAGS) -o $@ $(LDFLAGS)
	./$@	

clean:
	rm -rf ./obj
	rm -f $(OUTPUT)-{release,debug,test}
