use dyn_clone::DynClone;

use crate::{
    DirectionEnum, ReportField,
    core::raw::{Cmd, PlaceHolder, RawCapsule, RawChamber, Rawfield},
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

    pub fn new_with_hex(hex: &'static str, title: &str, value: &str) -> Self {
        Self {
            bytes: hex_util::hex_to_bytes(hex).unwrap(),
            title: title.into(),
            hex: hex.into(),
            value: value.into(),
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
        }
    }

    pub fn new_downstream(up_stream_capsule: &RawCapsule<T>) -> Self {
        Self {
            bytes: Vec::new(),
            hex: String::new(),
            field_details: Vec::new(),
            cmd: up_stream_capsule.get_cmd_clone(),
            device_no: up_stream_capsule.device_no.clone(),
            device_id: up_stream_capsule.device_id.clone(),
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Downstream,
        }
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
