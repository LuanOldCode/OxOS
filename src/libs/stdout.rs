use core::{fmt::Write};
use crate::drivers::sbi;

/// [en] Returns an implementation of Write trait for standard output
/// [pt-br] Retorna uma implementação do trait Write para a saída padrão
#[inline]
pub fn stdout() -> impl Write {
    sbi::StdOut
}
