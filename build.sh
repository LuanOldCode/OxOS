cargo build --release
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64imac-unknown-none-elf/release/OxOS