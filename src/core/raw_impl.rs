use crate::core::raw::Rawfield;

impl Rawfield {
    /// 一个构造函数，用于根据原始字节和翻译结果来创建Rawfield
    pub fn new(raw_bytes: &[u8], title: &'static str, value: &'static str) -> Self {
        Self {
            title: title.into(),
            hex: hex::encode_upper(raw_bytes), // 编码为Hex字符串
            value: value.into(),
        }
    }

    pub fn new_with_hex(hex: &'static str, title: &'static str, value: &'static str) -> Self {
        Self {
            title: title.into(),
            hex: hex.into(),
            value: value.into(),
        }
    }
}
