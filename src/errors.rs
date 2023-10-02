#[derive(Debug)]
pub enum SubroutineError {
    InvalidAddress(u16),
    StackOverflow,
    // ...
}

pub enum RegisterError {
    InvalidRegister(u8),
    StackOverflow,
    // ...
}