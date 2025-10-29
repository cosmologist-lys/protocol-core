use dyn_clone::DynClone;

use crate::{CrcType, DirectionEnum, MsgTypeEnum, ReportField, core::RW};

// 报文帧字段 最小解析单位
#[derive(Debug, Clone, Default)]
pub struct Rawfield {
    pub bytes: Vec<u8>,
    pub title: String,
    pub hex: String,
    pub value: String,
}

// 占位符
#[derive(Debug, Clone, Default)]
pub struct PlaceHolder {
    pub tag: String,
    pub pos: usize,
    pub start_index: usize,
    pub end_index: usize,
}

// 报文上/下行解析 处理之后的结果 第二小解析单位，比RawField大
#[derive(Debug, Clone)]
pub struct RawCapsule<T: Cmd> {
    pub bytes: Vec<u8>,
    pub hex: String,
    pub field_details: Vec<ReportField>,
    pub cmd: Option<T>,
    pub device_no: Option<String>,
    pub device_id: Option<String>,
    // 临时二进制存放处
    pub temp_bytes: Vec<u8>,
    pub direction: DirectionEnum,
    pub success: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RawChamber<T: Cmd> {
    pub upstream: Option<RawCapsule<T>>,
    pub downstream: Option<RawCapsule<T>>,
}

pub trait Cmd: DynClone {
    fn code(&self) -> String;

    fn title(&self) -> String;

    fn direction(&self) -> DirectionEnum {
        DirectionEnum::Both
    }

    fn rw(&self) -> Option<RW> {
        Some(RW::Write)
    }

    fn msg_type(&self) -> Option<MsgTypeEnum> {
        Some(MsgTypeEnum::DeviceParamSetting)
    }

    fn is_success(&self) -> bool {
        true
    }
}

pub trait ProtocolConfig {
    fn head_tag(&self) -> String;

    fn tail_tag(&self) -> String;

    fn crc_mode(&self) -> CrcType;

    fn crc_index(&self) -> (u8, u8);

    fn length_index(&self) -> (u8, u8);
}
