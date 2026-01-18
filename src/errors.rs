use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid server response")]
    InvalidResponse,

    #[error("Not implemented: {feature}")]
    NotImplemented { feature: &'static str },

    #[error("Invalid header: expected 0x{expected:02X}, found 0x{found:02X}")]
    InvalidHeader { expected: u8, found: u8 },

    #[error("Unexpected answer id: expected {expected}, found {found}")]
    UnexpectedAnswerID { expected: i32, found: i32 },

    #[error("Invalid server type received")]
    InvalidServerType,

    #[error("Invalid server environment received")]
    InvalidServerEnvironment,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
