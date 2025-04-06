#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut x = 0;
    loop {
        x += 1;
        unsafe { core::ptr::write_volatile(0x1000_0000 as *mut u32, x); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}