# A sample.
CC ?= cc
CFLAGS := -O2 -Wall
SOURCES = $(wildcard src/*.c)
OBJECTS = $(SOURCES:.c=.o)
TARGET = sample

ifeq ($(DEBUG),1)
CFLAGS += -g
endif

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

test: all
	./$(TARGET) --test

clean:
	rm -f $(OBJECTS) $(TARGET)

include config.mk
