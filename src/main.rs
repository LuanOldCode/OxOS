#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

pub mod uart;

static LOGO: &str = r"
 _____              __   ____
/\  __`\          /'__`\/\  _`\
\ \ \/\ \   __  _/\ \/\ \ \,\ \_\
 \ \ \ \ \ /\ \/'\ \ \ \ \/_\__ \
  \ \ \_\ \\/>  </\ \ \_\ \/\ \ \ \
   \ \_____\/\_/\_\\ \____/\ `\____\
    \/_____/\//\/_/ \/___/  \/_____/

------------------------------------
       OxOS - Version 0.0.001
------------------------------------
";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut my_uart = uart::Uart::new(0x1000_0000);

	my_uart.init();

    let _ = my_uart.write_str(LOGO);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}