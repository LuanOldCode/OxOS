// use core::{arch::asm, panic::PanicInfo};
// use crate::println;

// /// [en] Panic handler that prints panic information and enters an infinite wait loop
// /// [pt-br] Manipulador de pânico que imprime informações do pânico e entra em um loop infinito de espera
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     println!("\n\x1b[m#### KERNEL PANIC ####\n{}", _info);
//     loop {
//         unsafe {
//             asm!("wfi");
//         }
//     }
// }
