#![no_std]
pub mod drivers;
pub mod vga;
mod libs;

use core::{arch::asm, fmt::Write, panic::PanicInfo};
use crate::drivers::sbi;
/// [en] Panic handler that prints panic information and enters an infinite wait loop
/// [pt-br] Manipulador de pânico que imprime informações do pânico e entra em um loop infinito de espera
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\n\x1b[m#### KERNEL PANIC ####\n{}", _info);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

/// [en] Macro to define the architecture entry point with initialization
/// [pt-br] Macro para definir o ponto de entrada da arquitetura com inicialização
#[macro_export]
macro_rules! arch_entry {
    ($start:ident) => {
        use core::{
            arch::{asm, global_asm},
            ffi::c_void,
        };

        unsafe extern "C" {
            static __bss: c_void;
            static __bss_end: c_void;
            static __stack_top: c_void;
        }

        global_asm!(
            "
        .section .text.boot
        .global boot
        .align 4
        boot:
            la sp, __stack_top
            j _arch_riscv_start
        "
        );

        /// [en] RISC-V architecture start function that initializes BSS and calls the main entry point
        /// [pt-br] Função de início da arquitetura RISC-V que inicializa BSS e chama o ponto de entrada principal
        #[unsafe(no_mangle)]
        unsafe fn _arch_riscv_start(_hartid: usize, _dtb: usize) -> ! {
            unsafe {
                let bss = &__bss as *const _ as *mut u8;
                let bss_end = &__bss_end as *const _;
                bss.write_bytes(0, bss_end as usize - bss as usize);
            };

            $start();

            #[allow(unreachable_code)]
            loop {
                unsafe {
                    asm!("wfi");
                }
            }
        }
    };
}

/// [en] Returns an implementation of Write trait for standard output
/// [pt-br] Retorna uma implementação do trait Write para a saída padrão
#[inline]
pub fn stdout() -> impl Write {
    sbi::StdOut
}
