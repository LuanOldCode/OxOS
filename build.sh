#rm -rf -r target
#cargo clean
#cargo build --release
riscv-none-embed-objdump -D target/riscv64imac-unknown-none-elf/release/kernel > target/riscv64imac-unknown-none-elf/release/kernel.sym
qemu-system-riscv64 -M virt -device VGA -serial mon:stdio -kernel target/riscv64imac-unknown-none-elf/release/kernel