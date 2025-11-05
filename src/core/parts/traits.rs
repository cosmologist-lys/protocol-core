use std::collections::HashMap;

use crate::{
    CrcType, DirectionEnum, FieldType, MsgTypeEnum, ProtocolError, ProtocolResult, Rawfield,
    Writer,
    core::{RW, parts::transport_pair::TransportPair},
    hex_util,
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

// 下行参数设置，针对单个帧字段
pub trait EncodingParams {
    fn code(&self) -> String; // 唯一标识符
    fn title(&self) -> String; // 字段名称
    fn byte_length(&self) -> u8; // 字节长度，0表示变长，1表示固定长度
    // 命令码
    fn cmd_code(&self) -> String {
        String::new()
    }
    fn field_type(&self) -> FieldType; // 实际类型
    // 前端输入类型，string,int,float
    fn input_field_type(&self) -> String {
        match self.field_type() {
            FieldType::StringOrBCD | FieldType::Ascii => "string".to_string(),
            FieldType::Float | FieldType::Double => "float".to_string(),
            _ => "int".to_string(),
        }
    }
    fn default_value(&self) -> String {
        String::new()
    }
    fn default_hex(&self) -> String {
        String::new()
    }

    // 是否翻转。true=小端 false=大端
    fn swap(&self) -> bool {
        false
    }

    // 是否必填
    fn required(&self) -> bool {
        true
    }

    // 根据实现的以上的trait规则，自动生成bytes
    fn to_bytes(&self, input: &str) -> ProtocolResult<Vec<u8>> {
        // 步骤1: 确定输入值
        let mut bytes: Vec<u8>;
        let ft = self.field_type();
        if input.is_empty() {
            // 情况1: 输入为空
            let default_hex = self.default_hex();
            let default_value = self.default_value();

            if !default_hex.is_empty() {
                // 1-1: 使用 default_hex
                bytes = hex_util::hex_to_bytes(&default_hex)?;
            } else if !default_value.is_empty() {
                // 1-1: 使用 default_value 并根据 FieldType 编码
                bytes = ft.encode(&default_value)?;
            } else {
                // 1-2: 两者都为空且该值是必须的，抛错
                if self.required() {
                    return Err(ProtocolError::CommonError(format!(
                        "Field '{}' is required but no value provided",
                        self.code()
                    )));
                }
                bytes = Vec::new();
            }
        } else {
            // 情况2: 输入有值
            bytes = ft.encode(input)?;
        }

        // 步骤2: 调整字节长度
        let expected_length = self.byte_length() as usize;
        let actual_length = bytes.len();

        if expected_length > 0 && actual_length != expected_length {
            if actual_length > expected_length {
                // 长度超过，从低位开始保留，抛弃高位
                // 例如: [0x77, 0xFF, 0xBD, 0x23] 保留2字节 -> [0xBD, 0x23]
                bytes = bytes[(actual_length - expected_length)..].to_vec();
            } else {
                // 长度不足，在高位补0
                // 例如: [0xBD, 0x23] 扩展到4字节 -> [0x00, 0x00, 0xBD, 0x23]
                let mut padded = vec![0u8; expected_length - actual_length];
                padded.extend_from_slice(&bytes);
                bytes = padded;
            }
        }

        // 步骤3: 根据 swap 标志进行高低位交换
        if self.swap() {
            bytes = hex_util::swap_bytes(&bytes)?;
        }

        Ok(bytes)
    }
}

/// 用于修饰实现了 EncodingParams 的枚举类型
/// 提供枚举级别的操作接口
pub trait EncodingDefinition<T: EncodingParams>: Sized {
    /// 获取枚举的所有变体
    fn variants() -> Vec<T>;

    /// 获取枚举的所有变体的映射
    fn variants_map() -> HashMap<String, T>;

    // 只要定义好了trait:EncodingParams，它就会自动实现它的to_bytes方法。
    // 这里只需要挨个调用EncodingParams.to_bytes方法就好了
    // 返回的是整个处理的总长度
    fn auto_process(
        &self,
        definitions: Vec<Box<dyn EncodingParams>>, // list of EncodingParams
        params: &HashMap<String, String>,          // 输入的下发参数map
        writer: &mut Writer,
    ) -> ProtocolResult<u16> {
        let mut length: usize = 0;
        for definition in definitions {
            let code = definition.code();
            let title = definition.title();
            // 是否必须
            let require = definition.required();
            if let Some(input) = params.get(&code) {
                let bytes = definition.to_bytes(input)?;
                length += bytes.len();
                writer.write(|| {
                    let rf = Rawfield::new(&bytes, title, input.to_string());
                    Ok(rf)
                })?;
            } else if require {
                return Err(ProtocolError::CommonError(
                    "input params require but found empty".into(),
                ));
            }
        }
        Ok(length as u16)
    }
}
