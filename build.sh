cargo build --release
qemu-system-riscv64 -M virt -device virtio-vga -serial mon:stdio -kernel target/riscv64gc-unknown-none-elf/release/OxOS
