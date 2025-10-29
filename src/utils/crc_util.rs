use rust_decimal::prelude::ToPrimitive;

use crate::{
    defi::{
        ProtocolResult,
        crc_enum::{CrcCalculator, CrcType},
        error::ProtocolError,
    },
    utils::hex_util,
};

pub fn calculate_from_hex(crc_type: CrcType, hex: &str) -> ProtocolResult<String> {
    crc_type.calculate_from_hex(hex)
}

pub fn calculate_from_bytes(crc_type: CrcType, bytes: &[u8]) -> ProtocolResult<u16> {
    crc_type.calculate(bytes)
}

pub fn compare_crc(crc1: &str, crc2: u16) -> ProtocolResult<()> {
    let crc1_u16 = hex_util::hex_to_u16(crc1)?;
    if crc1_u16 == crc2 {
        Ok(())
    } else {
        let mut temp = hex_util::hex_to_bytes(crc1)?;
        temp.reverse();
        let crc1_c = hex_util::bytes_to_hex(&temp)?;
        let crc1_u16 = hex_util::hex_to_i32(crc1_c.as_str())?;
        let calc_ori_crc = crc1_u16.to_u16().unwrap();
        match calc_ori_crc == crc2 {
            true => Ok(()),
            false => Err(ProtocolError::CrcError {
                ori_crc: calc_ori_crc,
                calc_crc: crc2,
            }),
        }
    }
}
