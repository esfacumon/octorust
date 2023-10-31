#[derive(Debug)]
pub enum SubroutineError {
    InvalidAddress(u16),
    StackOverflow,
}


#[derive(Debug)]
pub enum RegisterError {
    InvalidRegister(u8),
}
