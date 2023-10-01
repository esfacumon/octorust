#[derive(Debug)]
pub enum SubroutineError {
    InvalidAddress(u16),
    StackOverflow,
    // ...
}