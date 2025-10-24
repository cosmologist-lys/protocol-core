use crate::defi::{ProtocolResult, crc_enum::CrcType, error::ProtocolError};

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

pub enum DirectionEnum {
    Upstream,
    Downstream,
    Both,
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

pub enum MsgTypeEnum {
    SignIn,             //("signin", "注册"),
    DataReport,         //("data_report", "数据上报"),
    ValveOperation,     //("valve_operation", "阀门控制"),
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
