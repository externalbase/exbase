LIB_NAME := exbase

CRATE_DIR := ../src/rust
WORKSPACEDIR := ../src/c
LDIR := $(CRATE_DIR)/target/release
OUTDIR := $(WORKSPACEDIR)/bin

CC := gcc
CFLAGS := -I$(WORKSPACEDIR)/include

.PHONY: all rust c clean

all: c

rust: 
	cd $(CRATE_DIR) && \
		cargo rustc --features=ffi --release --crate-type cdylib

c: rust
	@mkdir -p $(OUTDIR)
	$(CC) $(WORKSPACEDIR)/example/main.c -o $(OUTDIR)/example $(CFLAGS) -L$(LDIR) -l$(LIB_NAME)
	@cp $(LDIR)/lib$(LIB_NAME).so $(OUTDIR)/

clean:
	@rm -rf $(OUTDIR)