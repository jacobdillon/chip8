TARGETS=disassemble
SRCS=$(patsubst %,%.c,$(TARGETS))
CC=gcc
CFLAGS=-Wall -Wextra -Wpedantic -g

all : $(TARGETS)

$(TARGETS): %: %.c
	$(CC) $(CFLAGS) -o $@ $<

clean: 
	@rm -f $(TARGETS) *.o a.out

