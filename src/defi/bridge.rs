use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Cmd, ProtocolError, ProtocolResult, RawCapsule, RawChamber, core::parts::rawfield::Rawfield,
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
pub struct JniRequest {
    pub(crate) device_id: String,
    pub(crate) device_no: String,
    pub(crate) msgt_type: String,
    pub(crate) cmd_code: String,
    pub(crate) hex: String,
    pub(crate) uri: String,
    pub(crate) params: HashMap<String, String>,
}

impl JniRequest {
    pub fn new(
        device_id: String,
        device_no: String,
        msgt_type: String,
        cmd_code: String,
        hex: String,
        uri: String,
        params: HashMap<String, String>,
    ) -> Self {
        JniRequest {
            device_id,
            device_no,
            msgt_type,
            cmd_code,
            hex,
            uri,
            params,
        }
    }

    pub fn to_bytes(&self) -> ProtocolResult<Vec<u8>> {
        let json_string =
            serde_json::to_string(self).map_err(|e| ProtocolError::CommonError(e.to_string()))?;
        Ok(json_string.into_bytes())
    }

    pub fn from(data: &[u8]) -> ProtocolResult<Self> {
        let json_string =
            std::str::from_utf8(data).map_err(|e| ProtocolError::CommonError(e.to_string()))?;
        let request = serde_json::from_str(json_string)
            .map_err(|e| ProtocolError::CommonError(e.to_string()))?;
        Ok(request)
    }

    // Getter methods
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn device_id_clone(&self) -> String {
        self.device_id.clone()
    }

    pub fn hex(&self) -> &str {
        &self.hex
    }

    pub fn hex_clone(&self) -> String {
        self.hex.clone()
    }

    pub fn device_no(&self) -> &str {
        &self.device_no
    }

    pub fn device_no_clone(&self) -> String {
        self.device_no.clone()
    }

    pub fn msgt_type(&self) -> &str {
        &self.msgt_type
    }

    pub fn msgt_type_clone(&self) -> String {
        self.msgt_type.clone()
    }

    pub fn cmd_code(&self) -> &str {
        &self.cmd_code
    }

    pub fn cmd_code_clone(&self) -> String {
        self.cmd_code.clone()
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn uri_clone(&self) -> String {
        self.uri.clone()
    }

    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }

    pub fn params_clone(&self) -> HashMap<String, String> {
        self.params.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JniResponse {
    pub(crate) success: bool,
    pub(crate) device_id: String,
    pub(crate) device_no: String,
    pub(crate) msgt_type: String,
    pub(crate) cmd_code: String,
    pub(crate) req_hex: String,
    pub(crate) rsp_hex: String,
    pub(crate) req_jsons: Vec<ReportField>,
    pub(crate) rsp_jsons: Vec<ReportField>,
}

impl JniResponse {
    pub fn to_bytes(&self) -> ProtocolResult<Vec<u8>> {
        let json_string =
            serde_json::to_string(self).map_err(|e| ProtocolError::CommonError(e.to_string()))?;
        Ok(json_string.into_bytes())
    }

    // Getter methods
    pub fn success(&self) -> bool {
        self.success
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn device_id_clone(&self) -> String {
        self.device_id.clone()
    }

    pub fn device_no(&self) -> &str {
        &self.device_no
    }

    pub fn device_no_clone(&self) -> String {
        self.device_no.clone()
    }

    pub fn msgt_type(&self) -> &str {
        &self.msgt_type
    }

    pub fn msgt_type_clone(&self) -> String {
        self.msgt_type.clone()
    }

    pub fn cmd_code(&self) -> &str {
        &self.cmd_code
    }

    pub fn cmd_code_clone(&self) -> String {
        self.cmd_code.clone()
    }

    pub fn req_hex(&self) -> &str {
        &self.req_hex
    }

    pub fn req_hex_clone(&self) -> String {
        self.req_hex.clone()
    }

    pub fn rsp_hex(&self) -> &str {
        &self.rsp_hex
    }

    pub fn rsp_hex_clone(&self) -> String {
        self.rsp_hex.clone()
    }

    pub fn req_jsons(&self) -> &[ReportField] {
        &self.req_jsons
    }

    pub fn req_jsons_clone(&self) -> Vec<ReportField> {
        self.req_jsons.clone()
    }

    pub fn rsp_jsons(&self) -> &[ReportField] {
        &self.rsp_jsons
    }

    pub fn rsp_jsons_clone(&self) -> Vec<ReportField> {
        self.rsp_jsons.clone()
    }

    // Setter methods
    pub fn set_success(&mut self, success: bool) {
        self.success = success;
    }

    pub fn set_device_id(&mut self, device_id: &str) {
        self.device_id = device_id.to_string();
    }

    pub fn set_device_no(&mut self, device_no: &str) {
        self.device_no = device_no.to_string();
    }

    pub fn set_msgt_type(&mut self, msgt_type: &str) {
        self.msgt_type = msgt_type.to_string();
    }

    pub fn set_cmd_code(&mut self, cmd_code: &str) {
        self.cmd_code = cmd_code.to_string();
    }

    pub fn set_req_hex(&mut self, req_hex: &str) {
        self.req_hex = req_hex.to_string();
    }

    pub fn set_rsp_hex(&mut self, rsp_hex: &str) {
        self.rsp_hex = rsp_hex.to_string();
    }

    pub fn set_req_jsons(&mut self, req_jsons: Vec<ReportField>) {
        self.req_jsons = req_jsons;
    }

    pub fn set_rsp_jsons(&mut self, rsp_jsons: Vec<ReportField>) {
        self.rsp_jsons = rsp_jsons;
    }

    // 上行的返回
    pub fn upstream_response<T: Cmd + Clone + 'static>(
        chamber: &RawChamber<T>,
    ) -> ProtocolResult<Self> {
        let device_id = chamber.device_id_clone().unwrap_or_default();
        let device_no = chamber.device_no_clone().unwrap_or_default();
        // 获取 cmd_code
        let cmd_code = chamber.cmd_code_clone();
        // 获取 upstream 的 hex 和 field_details
        let (req_hex, req_jsons) = if let Some(upstream) = chamber.upstream() {
            (upstream.hex_clone(), upstream.field_details_clone())
        } else {
            (String::new(), Vec::new())
        };
        // 获取 downstream 的 hex 和 field_details
        let (rsp_hex, rsp_jsons) = if let Some(downstream) = chamber.downstream() {
            (downstream.hex_clone(), downstream.field_details_clone())
        } else {
            (String::new(), Vec::new())
        };
        // msgt_type 暂时设置为空字符串，根据实际需求调整
        let msgt_type = String::new();
        Ok(Self {
            success: chamber.success(),
            device_id,
            device_no,
            msgt_type,
            cmd_code,
            req_hex,
            rsp_hex,
            req_jsons,
            rsp_jsons,
        })
    }

    // 下行的返回
    pub fn downstream_response<T: Cmd + Clone + 'static>(
        capsule: &RawCapsule<T>,
    ) -> ProtocolResult<Self> {
        // 获取 device_id 和 device_no
        let device_id = capsule.device_id_clone().unwrap_or_default();
        let device_no = capsule.device_no_clone().unwrap_or_default();

        // 获取 cmd_code (从 cmd 中提取)
        let cmd_code = capsule.cmd().map(|cmd| cmd.code()).unwrap_or_default();

        // 下行返回没有上行内容，req_hex 和 req_jsons 为空
        let req_hex = String::new();
        let req_jsons = Vec::new();

        // rsp_hex 和 rsp_jsons 对应 capsule 的数据
        let rsp_hex = capsule.hex_clone();
        let rsp_jsons = capsule.field_details_clone();

        // msgt_type 暂时设置为空字符串
        let msgt_type = String::new();

        Ok(Self {
            success: capsule.success(),
            device_id,
            device_no,
            msgt_type,
            cmd_code,
            req_hex,
            rsp_hex,
            req_jsons,
            rsp_jsons,
        })
    }
}
