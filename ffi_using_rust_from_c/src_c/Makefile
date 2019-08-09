# CC ?= gcc
CC ?= clang
CFLAGS += -Wall

# dynamic works, requires libfoo.dll
# LDFLAGS = -L"../target/release/" -lfoo
LDFLAGS = -L"../target/release/" -Wl,-Bdynamic -lfoo
# static does not work
# LDFLAGS = -L"../target/release/" -Wl,-Bstatic -lfoo

SRC_FILES = main.c
OBJ_FILES = $(patsubst %.c, %.o, $(SRC_FILES))

.PHONY: all, clean

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

all: $(OBJ_FILES)
	$(CC) -o main $(OBJ_FILES) $(LDFLAGS)
	ln -S libfoo.dll ../target/release/libfoo.dll

clean:
	rm -f main.exe *.{c,h}.orig libfoo.dll

