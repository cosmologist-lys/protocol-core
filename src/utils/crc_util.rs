use crate::defi::{
    ProtocolResult,
    crc_enum::{CrcCalculator, CrcType},
};

pub fn calculate_from_hex(crc_type: CrcType, hex: &str) -> ProtocolResult<String> {
    crc_type.calculate_from_hex(hex)
}

pub fn calculate_from_bytes(crc_type: CrcType, bytes: &[u8]) -> ProtocolResult<u16> {
    crc_type.calculate(bytes)
}
