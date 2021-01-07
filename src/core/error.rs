use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(u16),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnknownOpcode(op) => write!(f, "Unknown CHIP-8 opcode: {:#06x?}", op),
        }
    }
}
