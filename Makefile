CROSS_COMPILE=$(HOME)/opt/cross/bin/arm-none-eabi-

CC=$(CROSS_COMPILE)gcc
AS=$(CROSS_COMPILE)as
LD=$(CROSS_COMPILE)ld
OBJCOPY=$(CROSS_COMPILE)objcopy

rpi?=1

ifeq ($(rpi), 1)
	ASFLAGS=-mcpu=arm1176jzf-s
	CARGO_SUBCMD=xbuild
	cargo_target=arm-none-eabi.json
	cargo_target_dir=arm-none-eabi
	kernel_img=kernel.img
else
	ASFLAGS=-mcpu=cortex-a7
	cargo_target=armv7a-none-eabi
	kernel_img=kernel7.img
endif

ASFLAGS+=-DRPI=$(rpi)

CARGO_SUBCMD?=build
CARGO_CMD=cargo $(CARGO_SUBCMD)
cargo_target_dir?=$(cargo_target)

elf_target=rpi-xmodem-btldr.elf
rust_lib=target/$(cargo_target_dir)/release/librpi_xmodem_btldr.a

all: $(kernel_img)

$(kernel_img): $(elf_target)
	$(OBJCOPY) $< -O binary $@

$(elf_target): boot.o lib linker.ld
	$(LD) -T linker.ld -o $@ boot.o $(rust_lib) -nostdlib

lib:
	$(CARGO_CMD) --release --target $(cargo_target) --features rpi_$(rpi)

clean:
	rm -f boot.o $(elf_target) kernel*.img

.PHONY: lib clean
