pub mod crc_enum;
pub mod error;
pub mod bridge;

pub type ProtocolResult<T> = Result<T, error::ProtocolError>;
