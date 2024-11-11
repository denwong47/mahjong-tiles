/// Converts a value to a Chinese character.
pub trait HasChineseValue {
    /// Get the Chinese character for this value.
    fn chinese_value(&self) -> char;
}

impl HasChineseValue for u8 {
    fn chinese_value(&self) -> char {
        match self {
            1 => '一',
            2 => '二',
            3 => '三',
            4 => '四',
            5 => '五',
            6 => '六',
            7 => '七',
            8 => '八',
            9 => '九',
            _ => unreachable!("Unreachable: Value should be between 1 and 9"),
        }
    }
}
