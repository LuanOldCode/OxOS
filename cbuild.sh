rm -rf out
mkdir out
riscv-none-embed-gcc -ffreestanding -flto -nostartfiles -fomit-frame-pointer -nostdlib -fno-builtin -mcmodel=medany -march=rv64imac -mabi=lp64 -T "src/lds/virt.lds" -o out/OxOS.elf src/start.s src/ctest.c src/uart.c 
riscv-none-embed-objdump -D out/OxOS.elf > out/OxOS.sym=
qemu-system-riscv64 -machine virt -device VGA,vgamem_mb=64 -smp 1 -bios none -kernel out/OxOS.elf -d trace:pci_cfg_read,trace:pci_cfg_write,trace:vga_std_read_io,trace:vga_std_write_io,trace:vga_vbe_read,trace:vga_vbe_write,trace:vga_cirrus_read_io,trace:vga_cirrus_write_io