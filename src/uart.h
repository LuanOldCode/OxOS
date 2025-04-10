#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>

// #define reg_t uint32_t // RISCV32: register is 32bits
#define reg_t uint64_t // RISCV64: register is 64bits

// ref: https://www.activexperts.com/serial-port-component/tutorials/uart/
#define UART 0x10000000L
#define UART_THR (volatile uint8_t *)(UART + 0x00) // THR:transmitter holding register
#define UART_RHR (volatile uint8_t *)(UART + 0x00) // RHR:Receive holding register
#define UART_DLL (volatile uint8_t *)(UART + 0x00) // LSB of Divisor Latch (write mode)
#define UART_DLM (volatile uint8_t *)(UART + 0x01) // MSB of Divisor Latch (write mode)
#define UART_IER (volatile uint8_t *)(UART + 0x01) // Interrupt Enable Register
#define UART_LCR (volatile uint8_t *)(UART + 0x03) // Line Control Register
#define UART_LSR (volatile uint8_t *)(UART + 0x05) // LSR:line status register
#define UART_LSR_EMPTY_MASK 0x40                   // LSR Bit 6: Transmitter empty; both the THR and LSR are empty
#define UART_LSR_RX_READY_MASK (1 << 0)            // LSR Bit 0: 

#define UART_REGR(reg) (*(reg))
#define UART_REGW(reg, v) ((*reg) = (v))

#define EOF 0

int lib_getc(void);

int lib_putc(char ch);

void lib_puts(char *s);

int lib_vsnprintf(char * out, size_t n, const char* s, va_list vl);

int lib_vprintf(const char* s, va_list vl);

int lib_printf(const char* s, ...);

#undef printf
#define printf lib_printf