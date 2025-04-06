.section .text.entry
.global _start
_start:
  la sp, _stack_start    # Set up stack pointer
  call main              # Call Rust main()
  j .                    # Loop forever