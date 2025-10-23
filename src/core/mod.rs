use crate::defi::crc_enum::CrcType;

pub mod raw;
pub mod raw_impl;
pub mod reader;
pub mod writer;

pub trait ProtocolConfig {
    fn head_tag(&self) -> String;

    fn tail_tag(&self) -> String;

    fn crc_mode(&self) -> CrcType;

    fn crc_index(&self) -> (u8, u8);

    fn length_index(&self) -> (u8, u8);
}
