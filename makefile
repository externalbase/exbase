LIB_NAME := exbase

CRATE_DIR := src
EXAMPLE_DIR := examples/c
LDIR := $(CRATE_DIR)/target/release
OUTDIR := $(EXAMPLE_DIR)/bin

CC := gcc
CFLAGS := -I$(CRATE_DIR)

.PHONY: all rust c clean

all: c

rust: 
	cd $(CRATE_DIR) && \
		cargo rustc --features=ffi --release --crate-type cdylib

c: rust
	@mkdir -p $(OUTDIR)
	$(CC) $(EXAMPLE_DIR)/main.c -o $(OUTDIR)/example $(CFLAGS) -L$(LDIR) -l$(LIB_NAME)
	@cp $(LDIR)/lib$(LIB_NAME).so $(OUTDIR)/

clean:
	@rm -rf $(OUTDIR)