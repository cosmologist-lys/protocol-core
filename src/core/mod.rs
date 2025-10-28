use serde::{Deserialize, Serialize};

use crate::{
    Rawfield,
    defi::{ProtocolResult, crc_enum::CrcType, error::ProtocolError},
    handle_int, hex_util,
    math_util::{self, DecimalRoundingMode},
};

mod macro_plugin;
pub mod raw;
pub mod raw_impl;
pub mod reader;
pub mod writer;

// 单个帧字段的翻译: 翻译模式
#[derive(Debug, Clone)]
pub struct FieldConvertDecoder {
    pub title: String,         // 标题
    pub filed_type: FieldType, // 帧字段类型 不为空即是: 翻译模式。
    pub swap: bool,            // 是否高低换位，或true=小端 false=大端
    // 翻译之后的符号
    pub symbol: Option<Symbol>,
}

#[derive(Debug, Clone)]
// 单个帧字段的翻译：比较模式
pub struct FieldCompareDecoder {
    pub title: String,           // 标题
    pub swap: bool,              // 是否高低换位，或true=小端 false=大端
    pub compare_target: Vec<u8>, // 比较目标 不为空即是：比较模式
}

#[derive(Debug, Clone)]
pub struct FieldEnumDecoder {
    pub title: String,                      // 标题
    pub swap: bool,                         // 是否高低换位，或true=小端 false=大端
    pub enum_values: Vec<(String, String)>, // 枚举值
}

impl FieldConvertDecoder {
    pub fn new(title: &str, filed_type: FieldType, swap: bool) -> Self {
        FieldConvertDecoder {
            title: title.to_string(),
            filed_type,
            swap,
            symbol: None,
        }
    }

    pub fn set_symbol(&mut self, symbol: Symbol) {
        self.symbol = Some(symbol);
    }
}

impl FieldCompareDecoder {
    pub fn new(title: &str, compare_target: Vec<u8>, swap: bool) -> Self {
        FieldCompareDecoder {
            title: title.to_string(),
            compare_target,
            swap,
        }
    }
}

impl FieldEnumDecoder {
    pub fn new(title: &str, enum_values: Vec<(String, String)>, swap: bool) -> Self {
        FieldEnumDecoder {
            title: title.to_string(),
            enum_values,
            swap,
        }
    }
}

pub trait FieldTranslator {
    fn translate(&self, bytes: &[u8]) -> ProtocolResult<Rawfield>;
}

impl FieldTranslator for FieldConvertDecoder {
    fn translate(&self, bytes: &[u8]) -> ProtocolResult<Rawfield> {
        let mut copied_bytes = bytes.to_vec(); // 替代 clone_from_slice，更简单
        let input_bytes = if self.swap && bytes.len() > 1 {
            copied_bytes.reverse();
            copied_bytes
        } else {
            copied_bytes
        };
        let ft = &self.filed_type;
        let mut value = ft.convert(&input_bytes)?;
        // 如果有符号，拼接上去
        if self.symbol.is_some() {
            let symbol_some_clone = self.symbol.clone();
            let symbol = symbol_some_clone.unwrap();
            value += " ";
            value += symbol.tag().as_str();
        }
        Ok(Rawfield::new(bytes, self.title.clone(), value))
    }
}

impl FieldTranslator for FieldCompareDecoder {
    fn translate(&self, bytes: &[u8]) -> ProtocolResult<Rawfield> {
        let mut copied_bytes = bytes.to_vec(); // 替代 clone_from_slice，更简单
        let input_bytes = if self.swap && bytes.len() > 1 {
            copied_bytes.reverse();
            copied_bytes
        } else {
            copied_bytes
        };

        if input_bytes != self.compare_target {
            return Err(ProtocolError::CommonError(format!(
                "compare failed , target bytes : {:?} , expected bytes : {:?}",
                input_bytes, self.compare_target
            )));
        }
        let hex = hex_util::bytes_to_hex(&input_bytes)?;

        let rf = Rawfield::new(bytes, self.title.clone(), hex);

        Ok(rf)
    }
}

impl FieldTranslator for FieldEnumDecoder {
    fn translate(&self, bytes: &[u8]) -> ProtocolResult<Rawfield> {
        let mut copied_bytes = bytes.to_vec(); // 替代 clone_from_slice，更简单
        let input_bytes = if self.swap && bytes.len() > 1 {
            copied_bytes.reverse();
            copied_bytes
        } else {
            copied_bytes
        };

        let hex = hex_util::bytes_to_hex(&input_bytes)?;
        // 循环枚举值，比对 hex 是否匹配
        let value = self
            .enum_values
            .iter()
            // 找到第一个 hex 匹配的元组（忽略大小写？根据业务需求调整）
            .find(|&(enum_hex, _)| enum_hex == &hex)
            // 若找到，取第二个元素作为值；否则返回错误
            .map(|(_, enum_value)| enum_value.clone())
            .unwrap_or_else(|| hex.clone());

        // 构建并返回 Rawfield（hex 用输入字节的 hex，value 用匹配到的枚举值）
        let rf = Rawfield::new(bytes, self.title.clone(), value);
        Ok(rf)
        // todo
    }
}

pub trait ProtocolConfig {
    fn head_tag(&self) -> String;

    fn tail_tag(&self) -> String;

    fn crc_mode(&self) -> CrcType;

    fn crc_index(&self) -> (u8, u8);

    fn length_index(&self) -> (u8, u8);
}

#[derive(Debug, Clone)]
/// 字段类型
pub enum FieldType {
    StringOrBCD,      // 文字 or BCD
    UnsignedU8(f64),  // 正整数(缩小倍数) 1
    UnsignedU16(f64), // 正整数(缩小倍数) 2
    UnsignedU32(f64), // 正整数(缩小倍数) 3
    UnsignedU64(f64), // 正整数(缩小倍数) 4
    SignedI8(f64),    // 正负整数(缩小倍数) 1
    SignedI16(f64),   // 正负整数(缩小倍数) 2
    SignedI32(f64),   // 正负整数(缩小倍数) 3
    SignedI64(f64),   // 正负整数(缩小倍数) 4
    Float,            // 单精度4字节
    Double,           // 双精度8字节
    Ascii,            // ascii
}

impl FieldType {
    /// 根据FieldType将大端字节切片转换为字符串表示。
    pub fn convert(&self, bytes: &[u8]) -> ProtocolResult<String> {
        match self {
            FieldType::StringOrBCD => hex_util::bytes_to_hex(bytes),
            FieldType::UnsignedU8(scale) => handle_int!(u8, 1, bytes, *scale),
            FieldType::UnsignedU16(scale) => handle_int!(u16, 2, bytes, *scale),
            FieldType::UnsignedU32(scale) => handle_int!(u32, 4, bytes, *scale),
            FieldType::UnsignedU64(scale) => handle_int!(u64, 8, bytes, *scale),
            FieldType::SignedI8(scale) => handle_int!(i8, 1, bytes, *scale),
            FieldType::SignedI16(scale) => handle_int!(i16, 2, bytes, *scale),
            FieldType::SignedI32(scale) => handle_int!(i32, 4, bytes, *scale),
            FieldType::SignedI64(scale) => handle_int!(i64, 8, bytes, *scale),
            FieldType::Float => {
                if bytes.len() != 4 {
                    return Err(ProtocolError::ValidationFailed(format!(
                        "Invalid byte length for Float. Expected 4, got {}",
                        bytes.len()
                    )));
                }
                let value = f32::from_be_bytes(bytes.try_into().unwrap());
                Ok(value.to_string())
            }
            FieldType::Double => {
                if bytes.len() != 8 {
                    return Err(ProtocolError::ValidationFailed(format!(
                        "Invalid byte length for Double. Expected 8, got {}",
                        bytes.len()
                    )));
                }
                let value = f64::from_be_bytes(bytes.try_into().unwrap());
                Ok(value.to_string())
            }
            FieldType::Ascii => {
                // 检查是否所有字节都是ASCII
                if !bytes.is_ascii() {
                    return Err(ProtocolError::CommonError(
                        "Input bytes are not valid ASCII".to_string(),
                    ));
                }
                // 安全地将ASCII字节转换为String (不会失败)
                Ok(String::from_utf8(bytes.to_vec()).unwrap())
            }
        }
    }
}

#[derive(Debug, Clone)]
/// 方向
pub enum DirectionEnum {
    Upstream,   // 上行
    Downstream, // 下行
    Both,       // 可上可下
}

impl DirectionEnum {
    pub fn is_upstream(&self) -> bool {
        match self {
            DirectionEnum::Upstream => true,
            DirectionEnum::Downstream => false,
            DirectionEnum::Both => true,
        }
    }

    pub fn is_downstream(&self) -> bool {
        match self {
            DirectionEnum::Upstream => false,
            DirectionEnum::Downstream => true,
            DirectionEnum::Both => true,
        }
    }

    pub fn is_upstream_only(&self) -> bool {
        match self {
            DirectionEnum::Upstream => true,
            DirectionEnum::Downstream => false,
            DirectionEnum::Both => false,
        }
    }

    pub fn is_downstream_only(&self) -> bool {
        match self {
            DirectionEnum::Upstream => false,
            DirectionEnum::Downstream => true,
            DirectionEnum::Both => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgTypeEnum {
    #[serde(rename = "signin")]
    SignIn, //("signin", "注册"),
    #[serde(rename = "dataReport")]
    DataReport, //("data_report", "数据上报"),
    #[serde(rename = "valve_operation")]
    ValveOperation, //("valve_operation", "阀门控制"),
    BalanceSync,        //("sync_balance_centre_charging", "余额同步"),
    Recharge,           //("charge_operation", "充值"),
    UpdateGasPrice,     //("update_gas_price", "调价"),
    DeviceParamSetting, //("device_param_setting", "设备参数设置"),
    ServerTerminalOver, //("server_terminal_over", "服务器会话终止"),
    ErrorRespond,       //("error_respond","表端回复异常"),
    HeartBeat,          //("heart_beat","心跳包"),

    NotifyTerminal, //("notify_terminal","告知平台并下发结束帧")

    Unknown,
}

impl MsgTypeEnum {
    pub fn code(&self) -> String {
        match self {
            MsgTypeEnum::SignIn => "signin".to_string(),
            MsgTypeEnum::DataReport => "data_report".to_string(),
            MsgTypeEnum::ValveOperation => "valve_operation".to_string(),
            MsgTypeEnum::BalanceSync => "sync_balance_centre_charging".to_string(),
            MsgTypeEnum::Recharge => "charge_operation".to_string(),
            MsgTypeEnum::UpdateGasPrice => "update_gas_price".to_string(),
            MsgTypeEnum::DeviceParamSetting => "device_param_setting".to_string(),
            MsgTypeEnum::ServerTerminalOver => "server_terminal_over".to_string(),
            MsgTypeEnum::ErrorRespond => "error_respond".to_string(),
            MsgTypeEnum::HeartBeat => "heart_beat".to_string(),
            MsgTypeEnum::NotifyTerminal => "notify_terminal".to_string(),
            MsgTypeEnum::Unknown => "unknown".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            MsgTypeEnum::SignIn => "注册".to_string(),
            MsgTypeEnum::DataReport => "数据上报".to_string(),
            MsgTypeEnum::ValveOperation => "阀门控制".to_string(),
            MsgTypeEnum::BalanceSync => "余额同步".to_string(),
            MsgTypeEnum::Recharge => "充值".to_string(),
            MsgTypeEnum::UpdateGasPrice => "调价".to_string(),
            MsgTypeEnum::DeviceParamSetting => "设备参数设置".to_string(),
            MsgTypeEnum::ServerTerminalOver => "服务器会话终止".to_string(),
            MsgTypeEnum::ErrorRespond => "表端回复异常".to_string(),
            MsgTypeEnum::HeartBeat => "心跳包".to_string(),
            MsgTypeEnum::NotifyTerminal => "告知平台并下发结束帧".to_string(),
            MsgTypeEnum::Unknown => "未知".to_string(),
        }
    }

    pub fn code_of(code: &str) -> ProtocolResult<Self> {
        let f = match code {
            "signin" => MsgTypeEnum::SignIn,
            "data_report" => MsgTypeEnum::DataReport,
            "valve_operation" => MsgTypeEnum::ValveOperation,
            "sync_balance_centre_charging" => MsgTypeEnum::BalanceSync,
            "charge_operation" => MsgTypeEnum::Recharge,
            "update_gas_price" => MsgTypeEnum::UpdateGasPrice,
            "device_param_setting" => MsgTypeEnum::DeviceParamSetting,
            "server_terminal_over" => MsgTypeEnum::ServerTerminalOver,
            "error_respond" => MsgTypeEnum::ErrorRespond,
            "heart_beat" => MsgTypeEnum::HeartBeat,
            "notify_terminal" => MsgTypeEnum::NotifyTerminal,
            _ => MsgTypeEnum::Unknown,
        };
        match f {
            MsgTypeEnum::Unknown => Err(ProtocolError::CommError(
                crate::defi::error::comm_error::CommError::UnknownMsgType(code.to_string()),
            )),
            _ => Ok(f),
        }
    }
}

pub trait Command {
    fn code(&self) -> String;

    fn description(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Percent,
    Voltage,
    MilliVoltage,
    Amber,
    CubicMeter,
    Liter,
    MilliLiter,
    Celsius,
    MeterPerSec,
    MeterPerHour,
    PA,
    KPA,
    CubicMeterPerHour,
    CubicMeterPerSec,
    Yuan,
}

impl Symbol {
    pub fn tag(&self) -> String {
        match self {
            Symbol::Percent => "%".to_string(),
            Symbol::Voltage => "V".to_string(),
            Symbol::MilliVoltage => "mV".to_string(),
            Symbol::Amber => "A".to_string(),
            Symbol::CubicMeter => "m³".to_string(),
            Symbol::Liter => "L".to_string(),
            Symbol::MilliLiter => "mL".to_string(),
            Symbol::Celsius => "℃".to_string(),
            Symbol::MeterPerSec => "m/s".to_string(),
            Symbol::MeterPerHour => "m/h".to_string(),
            Symbol::PA => "Pa".to_string(),
            Symbol::KPA => "kPa".to_string(),
            Symbol::CubicMeterPerHour => "m³/h".to_string(),
            Symbol::CubicMeterPerSec => "m³/s".to_string(),
            Symbol::Yuan => "元".to_string(),
        }
    }
}
