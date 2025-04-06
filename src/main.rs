#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut my_uart = uart::Uart::new(0x1000_0000);

	my_uart.init();

    my_uart.put(b'H');
    my_uart.put(b'E');
    my_uart.put(b'L');
    my_uart.put(b'L');
    my_uart.put(b'O');
    my_uart.put(b'W');
    my_uart.put(b'O');
    my_uart.put(b'R');
    my_uart.put(b'L');
    my_uart.put(b'D');
    // Aqui vai o código de inicialização do sistema
    loop {}
}

// Obrigatório sem std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod uart;