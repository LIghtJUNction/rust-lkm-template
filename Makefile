# SPDX-License-Identifier: GPL-2.0

## Makefile for Rust LKM Template
## Modernized for 2026

KDIR ?= ../linux

.PHONY: all clean rust-analyzer help

all:
	@$(MAKE) LLVM=1 CLIPPY=1 -C $(KDIR) M=$(PWD) -j$(nproc)

clean:
	@$(MAKE) LLVM=1 -C $(KDIR) M=$(PWD) clean -j$(nproc)

rust-analyzer:
	@$(MAKE) LLVM=1 -C $(KDIR) M=$(PWD) rust-analyzer -j$(nproc)

help:
	@printf "Rust LKM Template Makefile\n"
	@printf "\n"
	@printf "Usage:\n"
	@printf "  make                Build the kernel module\n"
	@printf "  make clean          Remove built artifacts\n"
	@printf "  make rust-analyzer Generate rust-analyzer config\n"
	@printf "\n"
	@printf "Variables:\n"
	@printf "  KDIR=<path>        Kernel source directory (default: ../linux)\n"
