use crate::{
    CrcType, DirectionEnum, MsgTypeEnum, core::RW, core::parts::transport_pair::TransportPair,
};
use dyn_clone::DynClone;

/// Trait 定义了缓存中设备状态对象需要实现的方法。
/// 添加了 Clone, Send, Sync, 'static 约束以用于 moka 缓存。
pub trait Transport: Send + Sync + 'static {
    // 设备号(去除补位)
    fn device_no(&self) -> Option<TransportPair>;

    // 设备号(包含补位) - 可选，提供默认实现
    fn device_no_padding(&self) -> Option<TransportPair> {
        self.device_no() // 默认返回未补位的
    }

    // 设备号长度(hex-string or bcd-string)
    fn device_no_length(&self) -> Option<TransportPair>;

    // 上报类型
    fn report_type(&self) -> Option<TransportPair>;

    // 控制码
    fn control_field(&self) -> Option<TransportPair>;

    // 协议版本(hex-string or bcd-string)
    fn protocol_version(&self) -> Option<TransportPair>;

    // 设备类型(hex-string or bcd-string)
    fn device_type(&self) -> Option<TransportPair>;

    // 厂商代码(hex-string or bcd-string)
    fn factory_code(&self) -> Option<TransportPair>;

    // 上行消息序号(每次上行+1)
    fn upstream_count(&self) -> Option<TransportPair>;

    // 下行消息序号(每次下行+1)
    fn downstream_count(&self) -> Option<TransportPair>;

    // 加密类型(-1表示不加密。0表示使用默认密钥。>=1表示使用对应的密钥)
    fn cipher_slot(&self) -> i8 {
        -1 // 提供默认实现
    }

    // 是否使用加密
    fn use_cipher(&self) -> bool {
        self.cipher_slot() >= 0
    }
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
