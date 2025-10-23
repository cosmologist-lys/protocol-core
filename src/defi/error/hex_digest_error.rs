use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolDigestError {
    #[error("CRC checksum mismatch. Expected {expected}, but got {actual}.")]
    CrcMismatch { expected: u16, actual: u16 },

    #[error("Invalid frame start byte. Expected 0x{expected:02X}, but got 0x{actual:02X}.")]
    InvalidHead { expected: u8, actual: u8 },

    #[error("Invalid frame end byte. Expected 0x{expected:02X}, but got 0x{actual:02X}.")]
    InvalidTail { expected: u8, actual: u8 },

    #[error("Unknown or unsupported Data Object ID: {0}")]
    UnknownCommandId(&'static str),

    #[error("crc calculation error")]
    CRCCalculateError,
}
