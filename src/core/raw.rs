#[derive(Debug, Clone, Default)]
pub struct Rawfield {
    pub bytes: Vec<u8>,
    pub title: String,
    pub hex: String,
    pub value: String,
}

// 占位符
#[derive(Debug, Clone, Default)]
pub struct PlaceHolder {
    pub tag: String,
    pub pos: usize,
    pub start_index: usize,
    pub end_index: usize,
}
