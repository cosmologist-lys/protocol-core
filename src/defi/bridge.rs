use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Cmd, ProtocolError, ProtocolResult, RawChamber,
    core::{MsgTypeEnum, parts::rawfield::Rawfield},
    utils,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportField {
    pub name: String,
    pub code: String,
    pub value: String,
    pub alert: bool,
}

// 实现一个便捷的构造函数
impl ReportField {
    pub fn new(name: &str, code: &str, value: String) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            value,
            alert: false, // 默认为false
        }
    }
}

impl Rawfield {
    pub fn to_report_field(self) -> ReportField {
        let title = self.title;
        let code = utils::to_pinyin(&title);
        ReportField {
            name: title,
            code,
            value: self.value,
            alert: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JarDecodeResponse {
    pub success: bool,
    pub cmd_code: String,
    pub field_details: Vec<ReportField>,
    pub rsp_field_details: Vec<ReportField>,
    // 这才是最终要下行的数据hex
    pub rsp_data: String,
}

impl JarDecodeResponse {
    pub fn from_chamber<T: Cmd + Clone>(chamber: &RawChamber<T>) -> Self {
        let request_field_details = if let Some(upstream) = &chamber.upstream {
            upstream.field_details.clone()
        } else {
            Vec::new()
        };

        let (response_field_details, response_hex) = if let Some(downstream) = &chamber.downstream {
            (downstream.field_details.clone(), downstream.hex.clone())
        } else {
            (Vec::new(), String::new())
        };
        Self {
            success: chamber.success,
            cmd_code: chamber.cmd_code.clone(),
            field_details: request_field_details,
            rsp_field_details: response_field_details,
            rsp_data: response_hex,
        }
    }
    pub fn to_bytes(&self) -> ProtocolResult<Vec<u8>> {
        let json_string =
            serde_json::to_string(self).map_err(|e| ProtocolError::CommonError(e.to_string()))?;
        Ok(json_string.into_bytes())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JarEncodeRequest {
    pub msg_type: MsgTypeEnum,
    pub meter_addr_no: String,
    pub cmd_code: String,
    pub params: HashMap<String, String>,
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JarEncodeResponse {
    pub response_hex: String,
    pub rsp_field_detail: Vec<ReportField>,
    // 这才是最终要下行的数据hex
    pub rsp_data: String,
}
