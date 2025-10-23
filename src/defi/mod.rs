pub mod crc_enum;
pub mod error;

pub type ProtocolResult<T> = Result<T, error::ProtocolError>;
