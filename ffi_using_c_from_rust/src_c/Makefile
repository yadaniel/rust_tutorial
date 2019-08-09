CC ?= gcc
# CC ?= clang
# CFLAGS = -Wall -m64
CFLAGS = -Wall -m32

# installed gcc version exits with error with -m64 option
# clang accepts this option

SRC_FILES = foo.c bar.c
OBJ_FILES = $(patsubst %.c, %.o, $(SRC_FILES))

.PHONY: all, clean

all: $(OBJ_FILES)
	ar cr libfoobar.ar $(OBJ_FILES)

%.o:%.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ_FILES) *.{c,h}.orig libfoobar.ar


