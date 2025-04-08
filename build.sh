rm -rf -r target
cargo clean
cargo build --release
riscv-none-embed-objdump -D target/riscv64imac-unknown-none-elf/release/OxOS > target/riscv64imac-unknown-none-elf/release/OxOS.sym
qemu-system-riscv64 -machine virt -bios none -kernel target/riscv64imac-unknown-none-elf/release/OxOS -device VGA