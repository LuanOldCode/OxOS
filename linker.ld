ENTRY(_start)

SECTIONS
{
    . = 0x80000000; /* endereço de carga para QEMU spike ou virt */

    .text : {
        *(.text._start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
        *(COMMON)
    }
}
