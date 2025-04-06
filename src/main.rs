#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;
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

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("{}", LOGO);
    loop {}
}
