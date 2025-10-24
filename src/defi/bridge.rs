use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    core::{MsgTypeEnum, raw::Rawfield},
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
    pub rsp_field_detail: Vec<ReportField>,
    pub jar_size: u64,
    // 这才是最终要下行的数据hex
    pub rsp_data: String,
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
