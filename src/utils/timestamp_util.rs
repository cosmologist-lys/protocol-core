use crate::{
    defi::{
        ProtocolResult,
        error::{ProtocolError, hex_error::HexError},
    },
    utils::hex_util,
};

/// 定义了 BCD 时间戳的格式化类型
pub enum TimestampType {
    Year,
    YearMonth,
    YearMonthDay,
    YearMonthDayHour,
    YearMonthDayHourMin,
    YearMonthDayHourMinSec,
    HourMinSec,
}

const YEAR_PREFIX: &str = "20";

/// 核心转换函数：将 BCD 字节切片按指定格式转换为日期字符串
///
/// # Arguments
/// * `bcd_bytes` - BCD 格式的字节 (例如 `&[0x23, 0x05, 0x15]`)
/// * `timestamp_type` - 期望的时间戳格式
///
/// # Returns
/// * `ProtocolResult<String>` - 格式化后的字符串 (例如 "2023-05-15")
pub fn convert(bcd_bytes: &[u8], timestamp_type: TimestampType) -> ProtocolResult<String> {
    // 1. 将 BCD 字节转换为 BCD 字符串
    // (例如 &[0x23, 0x05, 0x15] -> "230515")
    let bcd_str = hex_util::bytes_to_hex(bcd_bytes)?;

    // 2. 校验是否为 BCD (全数字)
    if !hex_util::is_bcd(&bcd_str) {
        return Err(ProtocolError::HexError(HexError::NotBcd(bcd_str)));
    }

    // 3. 规范化：如果 BCD 字符串以 "20" 开头 (例如 "20230515")，
    //    则将其剥离为 "230515"，以便后续函数统一处理 "yy" 格式。
    //
    let ts = match bcd_str.starts_with(YEAR_PREFIX) {
        true => &bcd_str[YEAR_PREFIX.len()..],
        false => &bcd_str,
    };

    // 4. 根据类型分派给辅助函数
    let result = match timestamp_type {
        TimestampType::Year => convert_to_year(ts),
        TimestampType::YearMonth => convert_to_year_month(ts),
        TimestampType::YearMonthDay => convert_to_year_month_day(ts),
        TimestampType::YearMonthDayHour => convert_to_year_month_day_hour(ts),
        TimestampType::YearMonthDayHourMin => convert_to_year_month_day_hour_min(ts),
        TimestampType::YearMonthDayHourMinSec => convert_to_year_month_day_hour_min_sec(ts),
        TimestampType::HourMinSec => convert_to_hour_min_sec(ts),
    };

    Ok(result)
}

// --- 公共 API 别名 ---

pub fn to_year(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::Year)
}
pub fn to_year_month(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::YearMonth)
}
pub fn to_year_month_day(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::YearMonthDay)
}
pub fn to_year_month_day_hour(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::YearMonthDayHour)
}
pub fn to_year_month_day_hour_min(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::YearMonthDayHourMin)
}
pub fn to_year_month_day_hour_min_sec(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::YearMonthDayHourMinSec)
}
pub fn to_hour_min_sec(bcd_bytes: &[u8]) -> ProtocolResult<String> {
    convert(bcd_bytes, TimestampType::HourMinSec)
}

// --- 私有辅助函数 ---

fn convert_to_year(timestamp: &str) -> String {
    if timestamp.len() >= 2 {
        let yy = &timestamp[0..2];
        format!("{}{}", YEAR_PREFIX, yy)
    } else {
        timestamp.to_string()
    }
}

fn convert_to_year_month(timestamp: &str) -> String {
    if timestamp.len() >= 4 {
        let yy = &timestamp[0..2];
        let month = &timestamp[2..4];
        format!("{}{}-{}", YEAR_PREFIX, yy, month)
    } else {
        timestamp.to_string()
    }
}

fn convert_to_year_month_day(timestamp: &str) -> String {
    if timestamp.len() >= 6 {
        let yy = &timestamp[0..2];
        let month = &timestamp[2..4];
        let day = &timestamp[4..6];
        format!("{}{}-{}-{}", YEAR_PREFIX, yy, month, day)
    } else {
        timestamp.to_string()
    }
}

fn convert_to_year_month_day_hour(timestamp: &str) -> String {
    if timestamp.len() >= 8 {
        let yy = &timestamp[0..2];
        let month = &timestamp[2..4];
        let day = &timestamp[4..6];
        let hour = &timestamp[6..8];
        format!("{}{}-{}-{} {}", YEAR_PREFIX, yy, month, day, hour)
    } else {
        timestamp.to_string()
    }
}

fn convert_to_year_month_day_hour_min(timestamp: &str) -> String {
    if timestamp.len() >= 10 {
        let yy = &timestamp[0..2];
        let month = &timestamp[2..4];
        let day = &timestamp[4..6];
        let hour = &timestamp[6..8];
        let minute = &timestamp[8..10];
        format!(
            "{}{}-{}-{} {}:{}",
            YEAR_PREFIX, yy, month, day, hour, minute
        )
    } else {
        timestamp.to_string()
    }
}

fn convert_to_year_month_day_hour_min_sec(timestamp: &str) -> String {
    if timestamp.len() >= 12 {
        let yy = &timestamp[0..2];
        let month = &timestamp[2..4];
        let day = &timestamp[4..6];
        let hour = &timestamp[6..8];
        let minute = &timestamp[8..10];
        let second = &timestamp[10..12];
        format!(
            "{}{}-{}-{} {}:{}:{}",
            YEAR_PREFIX, yy, month, day, hour, minute, second
        )
    } else {
        timestamp.to_string()
    }
}

fn convert_to_hour_min_sec(timestamp: &str) -> String {
    if timestamp.len() >= 6 {
        let hour = &timestamp[0..2];
        let min = &timestamp[2..4];
        let sec = &timestamp[4..6];
        format!("{}:{}:{}", hour, min, sec)
    } else {
        timestamp.to_string()
    }
}
