use dyn_clone::DynClone;

use crate::{
    DirectionEnum, ProtocolError, ProtocolResult, ReportField,
    core::raw::{Cmd, PlaceHolder, RawCapsule, RawChamber, Rawfield, Transport, TransportCarrier},
    md5_digester::Md5Digester,
    utils::hex_util,
};

impl Rawfield {
    /// 一个构造函数，用于根据原始字节和翻译结果来创建Rawfield
    pub fn new(raw_bytes: &[u8], title: String, value: String) -> Self {
        Self {
            bytes: raw_bytes.to_vec(),
            title,
            hex: hex::encode_upper(raw_bytes), // 编码为Hex字符串
            value,
        }
    }

    pub fn new_with_hex(hex: &str, title: &str, value: String) -> Self {
        Self {
            bytes: hex_util::hex_to_bytes(hex).unwrap(),
            title: title.into(),
            hex: hex.into(),
            value,
        }
    }

    // pub fn hex_to_bytes(&self) -> ProtocolResult<Vec<u8>> {
    //     hex_util::hex_to_bytes(&self.hex)
    // }
}

impl PlaceHolder {
    pub fn new(tag: &str, pos: usize, start_index: usize, end_index: usize) -> Self {
        Self {
            tag: tag.into(),
            pos,
            start_index,
            end_index,
        }
    }

    /// 获取占位符的长度
    pub fn capacity(&self) -> usize {
        self.end_index - self.start_index
    }
}

impl<T: Cmd + 'static> RawCapsule<T> {
    pub fn new_upstream(bytes: &[u8]) -> Self {
        let hex = hex::encode_upper(bytes);
        Self {
            bytes: bytes.to_vec(),
            hex,
            field_details: Vec::new(),
            cmd: None,
            device_no: None,
            device_id: None,
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Upstream,
            success: true,
        }
    }

    pub fn new_downstream(cmd: T, device_no: &str, device_id: &str) -> Self {
        Self {
            bytes: Vec::new(),
            hex: String::new(),
            field_details: Vec::new(),
            cmd: Some(cmd),
            device_no: Some(device_no.into()),
            device_id: Some(device_id.into()),
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Downstream,
            success: true,
        }
    }

    // 获取一个唯一值。它由device_id和device_no一起组成进行md5加密
    pub fn get_unique_id(&self) -> ProtocolResult<String> {
        let device_no = if let Some(dn) = self.device_no.as_ref() {
            dn.clone()
        } else {
            "0".into()
        };

        let device_id = if let Some(dn) = self.device_id.as_ref() {
            dn.clone()
        } else {
            "0".into()
        };

        if device_no == "0" && device_id == "0" {
            return Err(ProtocolError::CommonError(
                "RawCapsule requires at least 1 of device_no and device_id but found both none"
                    .into(),
            ));
        }
        Md5Digester::digest_str_with_salt(&device_no, &device_id)
    }

    pub fn new_downstream_from_upstream(up_stream_capsule: &RawCapsule<T>) -> Self {
        let device_no = if up_stream_capsule.device_no.is_some() {
            up_stream_capsule.device_no.clone()
        } else {
            None
        };

        let device_id = if up_stream_capsule.device_id.is_some() {
            up_stream_capsule.device_id.clone()
        } else {
            None
        };
        Self {
            bytes: Vec::new(),
            hex: String::new(),
            field_details: Vec::new(),
            cmd: up_stream_capsule.get_cmd_clone(),
            device_no,
            device_id,
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Downstream,
            success: true,
        }
    }

    pub fn fail(&mut self) {
        self.success = false;
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn get_bytes_ref(&self) -> &[u8] {
        &self.bytes
    }

    pub fn is_upstream(&self) -> bool {
        self.direction.is_upstream()
    }

    pub fn is_downstream(&self) -> bool {
        self.direction.is_downstream()
    }

    pub fn set_device_id(&mut self, device_id: &str) {
        self.device_id = Some(device_id.into());
    }

    pub fn set_device_no(&mut self, device_no: &str) {
        self.device_no = Some(device_no.into());
    }

    pub fn set_cmd(&mut self, cmd: T) {
        self.cmd = Some(cmd);
    }

    pub fn set_temp_bytes(&mut self, bytes: &[u8]) {
        self.temp_bytes = bytes.to_vec();
    }

    pub fn get_temp_bytes_clone(&self) -> Vec<u8> {
        self.temp_bytes.clone().to_vec()
    }

    pub fn get_cmd_ref(&self) -> Option<&T> {
        self.cmd.as_ref()
    }

    pub fn get_cmd_clone(&self) -> Option<T>
    where
        T: DynClone, // 约束 T 必须支持动态克隆
    {
        self.cmd.as_ref().map(|cmd| {
            // 利用 dyn_clone 克隆 trait 对象
            dyn_clone::clone(cmd)
        })
    }
    pub fn set_fields(&mut self, fields: Vec<ReportField>) {
        self.field_details = fields;
    }

    pub fn append_fields(&mut self, fields: Vec<ReportField>) {
        self.field_details.extend(fields);
    }

    pub fn prepend_fields(&mut self, fields: Vec<ReportField>) {
        let mut new_fields = fields;
        new_fields.append(&mut self.field_details);
        self.field_details = new_fields;
    }
}

impl<T: Cmd> RawChamber<T> {
    pub fn new() -> Self {
        Self {
            upstream: None,
            downstream: None,
        }
    }

    pub fn update_upstream(&mut self, upstream: RawCapsule<T>) {
        self.upstream = Some(upstream);
    }

    pub fn update_downstream(&mut self, downstream: RawCapsule<T>) {
        self.downstream = Some(downstream);
    }
}

impl TransportCarrier {
    pub fn new(device_no: &str, device_no_padding: &str) -> Self {
        Self {
            device_no: device_no.to_string(),
            device_no_padding: device_no_padding.to_string(),
            protocol_version: "00".to_string(), // 示例值
            device_type: "07".to_string(),      // 示例值
            factory_code: "0000".to_string(),   // 示例值
            upstream_count: 0,
            downstream_count: 0,
            cipher_slot: -1,
        }
    }

    pub fn set_protocol_version(&mut self, version: &str) {
        self.protocol_version = version.to_string();
    }

    pub fn set_device_type(&mut self, device_type: &str) {
        self.device_type = device_type.to_string();
    }

    pub fn set_factory_code(&mut self, factory_code: &str) {
        self.factory_code = factory_code.to_string();
    }

    pub fn set_cipher_slot(&mut self, cipher_slot: i8) {
        self.cipher_slot = cipher_slot;
    }

    pub fn set_upstream_count(&mut self, count: usize) {
        self.upstream_count = count;
    }

    pub fn set_downstream_count(&mut self, count: usize) {
        self.downstream_count = count;
    }
}

impl Transport for TransportCarrier {
    fn device_no(&self) -> String {
        self.device_no.clone()
    }

    fn device_no_padding(&self) -> String {
        self.device_no_padding.clone()
    }

    fn protocol_version(&self) -> String {
        self.protocol_version.clone()
    }

    fn device_type(&self) -> String {
        self.device_type.clone()
    }

    fn factory_code(&self) -> String {
        self.factory_code.clone()
    }

    fn upstream_count(&self) -> usize {
        self.upstream_count
    }

    fn downstream_count(&self) -> usize {
        self.downstream_count
    }

    fn cipher_slot(&self) -> i8 {
        self.cipher_slot
    }
}
