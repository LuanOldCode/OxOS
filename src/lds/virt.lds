MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 512K
}

ENTRY(_start);

SECTIONS
{
  .text : {
    *(.text.entry)
    *(.text .text.*)
  } > RAM

  .rodata : {
    *(.rodata .rodata.*)
  } > RAM

  .data : {
    *(.data .data.*)
  } > RAM

  .bss : {
    *(.bss .bss.*)
    *(COMMON)
  } > RAM

  _stack_start = ORIGIN(RAM) + LENGTH(RAM);
}