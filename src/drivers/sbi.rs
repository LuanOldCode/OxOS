//! Wrapper library for RISC-V Supervisor Binary Interface (WIP)
//! [en] This library provides a Rust interface to RISC-V Supervisor Binary Interface (SBI) calls
//! [pt-br] Esta biblioteca fornece uma interface Rust para chamadas da Interface Binária do Supervisor RISC-V (SBI)

use core::arch::asm;

/// SBI extension ID
/// [en] Represents the SBI extension IDs for different operations
/// [pt-br] Representa os IDs de extensão SBI para diferentes operações
#[repr(transparent)]
pub struct Eid(pub usize);

impl Eid {
    /// [en] Puts a character to the console
    /// [pt-br] Coloca um caractere no console
    pub const CONSOLE_PUTCHAR: Self = Self(1);

    /// [en] Gets a character from the console
    /// [pt-br] Obtém um caractere do console
    pub const CONSOLE_GETCHAR: Self = Self(2);

    /// [en] Shuts down the system
    /// [pt-br] Desliga o sistema
    pub const SHUTDOWN: Self = Self(8);
}

/// [en] Outputs a single byte to the console using RISC-V SBI call
/// [pt-br] Envia um único byte para o console usando chamada SBI do RISC-V
#[inline]
pub fn putchar(ch: u8) {
    unsafe {
        asm!("ecall", in("a7") Eid::CONSOLE_PUTCHAR.0, in("a0") ch as usize);
    }
}

/// [en] Standard output implementation for the console, providing Write interface for formatted output
/// [pt-br] Implementação de saída padrão para o console, fornecendo interface Write para saída formatada
pub struct StdOut;

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            putchar(c);
        }
        Ok(())
    }
}

/// [en] Reads a single byte from the console through SBI, returns None if no input is available
/// [pt-br] Lê um único byte do console através do SBI, retorna None se nenhuma entrada estiver disponível
#[inline]
pub fn getchar() -> Option<u8> {
    unsafe {
        let c: usize;
        asm!("ecall", in("a7") Eid::CONSOLE_GETCHAR.0, lateout("a0") c);
        match c {
            0..=255 => Some(c as u8),
            _ => None,
        }
    }
}

/// [en] Shuts down the system through SBI call, never returns as system halts execution
/// [pt-br] Desliga o sistema através de chamada SBI, nunca retorna pois o sistema interrompe a execução
#[inline]
pub fn shutdown() -> ! {
    unsafe {
        asm!("ecall", in("a7") Eid::SHUTDOWN.0, options(noreturn));
    }
}
