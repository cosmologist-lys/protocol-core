// 占位符
#[derive(Debug, Clone, Default)]
pub struct PlaceHolder {
    pub tag: String,
    pub pos: usize,
    pub start_index: usize,
    pub end_index: usize,
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
