
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = write!(stdout(), $($arg)*);
    };
}

/// [en] Prints formatted text to standard output with a newline
/// [pt-br] Imprime texto formatado para a saída padrão com uma nova linha
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        let _ = writeln!(stdout(), $($arg)*);
    };
}
