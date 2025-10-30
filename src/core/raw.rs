use crate::{CrcType, DirectionEnum, MsgTypeEnum, ReportField, core::RW};
use dyn_clone::DynClone;

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
/// Trait 定义了缓存中设备状态对象需要实现的方法。
/// 添加了 Clone, Send, Sync, 'static 约束以用于 moka 缓存。
pub trait Transport: Send + Sync + 'static {
    // 设备号(去除补位)
    fn device_no(&self) -> String;

    // 设备号(包含补位) - 可选，提供默认实现
    fn device_no_padding(&self) -> String {
        self.device_no() // 默认返回未补位的
    }

    // 协议版本(hex-string or bcd-string)
    fn protocol_version(&self) -> String;

    // 设备类型(hex-string or bcd-string)
    fn device_type(&self) -> String;

    // 厂商代码(hex-string or bcd-string)
    fn factory_code(&self) -> String;

    // 上行消息序号(每次上行+1)
    fn upstream_count(&self) -> usize;

    // 下行消息序号(每次下行+1)
    fn downstream_count(&self) -> usize;

    // 加密类型(-1表示不加密。0表示使用默认密钥。>=1表示使用对应的密钥)
    fn cipher_slot(&self) -> i8 {
        -1 // 提供默认实现
    }

    // 是否使用加密
    fn use_cipher(&self) -> bool {
        self.cipher_slot() >= 0
    }
}

// 派生 Clone 是最简单的满足 moka 要求的方式
#[derive(Debug, Clone)]
pub struct TransportCarrier {
    pub(in crate::core) device_no: String,
    pub(in crate::core) device_no_padding: String,
    pub(in crate::core) protocol_version: String,
    pub(in crate::core) device_type: String,
    pub(in crate::core) factory_code: String,
    pub(in crate::core) upstream_count: usize,
    pub(in crate::core) downstream_count: usize,
    pub(in crate::core) cipher_slot: i8,
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
