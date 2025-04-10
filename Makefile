TARGET=riscv64gc-unknown-none-elf
KERNEL_NAME=OxOS
BUILD_DIR=target/$(TARGET)/release

.PHONY: all build run clean

all: build run

build:
	cargo build --release --target $(TARGET)

run: build
	qemu-system-riscv64 -M virt -device virtio-vga -serial mon:stdio -kernel $(BUILD_DIR)/$(KERNEL_NAME)

clean:
	cargo clean
