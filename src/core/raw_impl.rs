use crate::{
    core::raw::{PlaceHolder, Rawfield},
    defi::ProtocolResult,
    utils::hex_util,
};

impl Rawfield {
    /// 一个构造函数，用于根据原始字节和翻译结果来创建Rawfield
    pub fn new(raw_bytes: &[u8], title: &str, value: &str) -> Self {
        Self {
            title: title.into(),
            hex: hex::encode_upper(raw_bytes), // 编码为Hex字符串
            value: value.into(),
        }
    }

    pub fn new_with_hex(hex: &'static str, title: &str, value: &str) -> Self {
        Self {
            title: title.into(),
            hex: hex.into(),
            value: value.into(),
        }
    }

    pub fn hex_to_bytes(&self) -> ProtocolResult<Vec<u8>> {
        hex_util::hex_to_bytes(&self.hex)
    }
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
