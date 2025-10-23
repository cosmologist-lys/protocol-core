use crate::defi::{ProtocolResult, error::ProtocolError};

/**
 * 辅助函数：清理 hex 字符串 (trim, strip "0x")
 */
fn clean_hex_str(hex: &str) -> &str {
    let trimmed = hex.trim();
    trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed)
}

pub fn hex_to_bytes(s: &str) -> ProtocolResult<Vec<u8>> {
    ensure_is_machine_code(s)?;
    hex::decode(s).map_err(|_| ProtocolError::NotHex(s.into()))
}

/**
 * byte[] -> hex-string (小写)
 * @param bytes   字节切片
 * @return        hex-string
 */
pub fn bytes_to_hex(bytes: &[u8]) -> ProtocolResult<String> {
    // `hex::encode` 默认转换为小写
    Ok(hex::encode_upper(bytes))
}

/**
 * hex -> f64 (双精度 8 字节)
 * IEEE754标准
 */
pub fn hex_to_f64(hex: &str) -> ProtocolResult<f64> {
    ensure_is_machine_code(hex)?;
    // 1. 解析 hex。如果 hex 无效 (奇数长度或非法字符)，
    //    `hex_to_bytes` 会返回 Err(ProtocolError::NotHex)
    let bytes = hex_to_bytes(hex)?;

    // 2. 检查长度并转换
    //    Rust 的 `try_into` 是将 Vec<u8> 转换为 [u8; 8] 的
    //    最地道、最高效的方式。如果 `bytes.len() != 8`，
    //    它会失败，我们将其映射到自定义错误。
    let bytes_array: [u8; 8] = bytes.try_into().map_err(|vec: Vec<u8>| {
        // `vec` 是转换失败时返回的原始 Vec
        ProtocolError::InvalidFloatLength {
            expected: 8,
            actual: vec.len(),
        }
    })?;

    // 3. 从大端序 (Big-Endian) 字节创建 f64
    Ok(f64::from_be_bytes(bytes_array))
}

/**
 * hex -> f32 (单精度 4 字节)
 * IEEE754标准
 */
pub fn hex_to_f32(hex: &str) -> ProtocolResult<f32> {
    ensure_is_machine_code(hex)?;
    let bytes = hex_to_bytes(hex)?;

    // 2. 检查长度并转换 (同上, 目标为 4 字节)
    let bytes_array: [u8; 4] =
        bytes
            .try_into()
            .map_err(|vec: Vec<u8>| ProtocolError::InvalidFloatLength {
                expected: 4,
                actual: vec.len(),
            })?;

    // 3. 从大端序 (Big-Endian) 字节创建 f32
    Ok(f32::from_be_bytes(bytes_array))
}

/**
 * hex -> f64 (自动判断 f32 或 f64)
 *
 * 根据字节长度 (4 or 8) 转换。
 * 如果输入是 4 字节 (f32)，它将被提升 (cast) 为 f64。
 */
pub fn hex_to_f32_or_f64(hex: &str) -> ProtocolResult<f64> {
    ensure_is_machine_code(hex)?;
    let bytes = hex_to_bytes(hex)?;

    // Rust 的 `match` 语句非常适合处理这种情况
    match bytes.len() {
        8 => {
            // 长度为 8，执行 f64 逻辑
            // 此时 .unwrap() 是安全的, 因为我们已检查 len == 8
            let bytes_array: [u8; 8] = bytes.try_into().unwrap();
            Ok(f64::from_be_bytes(bytes_array))
        }
        4 => {
            // 长度为 4，执行 f32 逻辑
            // 此时 .unwrap() 是安全的, 因为我们已检查 len == 4
            let bytes_array: [u8; 4] = bytes.try_into().unwrap();
            let f32_val = f32::from_be_bytes(bytes_array);
            // 对应 Java 的 `return hex2Float(hex);`
            // (Java 会自动将 float 提升为 double 返回值)
            Ok(f32_val as f64)
        }
        actual_len => {
            // 其他所有长度都是错误的
            // 对应 Java: throw new IllegalArgumentException(...)
            Err(ProtocolError::InvalidFloatLengthEither { actual: actual_len })
        }
    }
}

/**
 * f32 (单精度) -> hex-string (大写)
 * IEEE754标准
 */
pub fn f32_to_hex(number: f32) -> ProtocolResult<String> {
    // 1. 获取 f32 的大端序字节 [u8; 4]
    let bytes = number.to_be_bytes();

    // 2. 将字节编码为大写 hex 字符串
    //    这个操作不会失败，所以我们直接 Ok()
    Ok(hex::encode_upper(bytes))
}

/**
 * f64 (双精度) -> hex-string (大写)
 * IEEE754标准
 */
pub fn f64_to_hex(number: f64) -> ProtocolResult<String> {
    // 1. 获取 f64 的大端序字节 [u8; 8]
    let bytes = number.to_be_bytes();
    // 2. 将字节编码为大写 hex 字符串
    Ok(hex::encode_upper(bytes))
}

/**
 * f64 -> hex-string (根据指定的字节长度 4 或 8)
 * (对应 Java floatOrDouble2Hex)
 */
pub fn f64_to_hex_by_len(number: f64, byte_length: usize) -> ProtocolResult<String> {
    match byte_length {
        4 => {
            // 对应 Java: return float2Hex((float) number);
            // 1. 将 f64 转换为 f32 (这可能会损失精度)
            let num_f32 = number as f32;
            // 2. 调用 f32 的转换逻辑
            f32_to_hex(num_f32)
        }
        8 => {
            // 对应 Java: return double2Hex(number);
            f64_to_hex(number)
        }
        actual_len => {
            // 对应 Java: throw new JarException(...)
            // 我们重用之前定义的错误类型
            Err(ProtocolError::InvalidFloatLengthEither { actual: actual_len })
        }
    }
}

/**
 * hex -> u32 (无符号 32-bit 整数)
 *
 * 我们选择地道的方式。
 */
pub fn hex_to_u32(hex: &str) -> ProtocolResult<u32> {
    ensure_is_machine_code(hex)?;
    let v = clean_hex_str(hex);
    // 限制 8 个字符 (4 字节)
    if v.len() > 8 {
        return Err(ProtocolError::HexLengthError {
            context: "u32",
            max_chars: 8,
            actual_chars: v.len(),
        });
    }
    if v.is_empty() {
        return Ok(0);
    }
    u32::from_str_radix(v, 16).map_err(|e| ProtocolError::HexParseError {
        context: "u32",
        reason: e.to_string(),
    })
}

/**
 * hex -> i16 (有符号 16-bit 整数)
 *
 * 即将 hex 视为无符号数, 然后按位重解释为有符号数。
 * (例如: "FFFF" -> 65535 (u16) -> -1 (i16))
 */
pub fn hex_to_i16(hex: &str) -> ProtocolResult<i16> {
    ensure_is_machine_code(hex)?;
    let v = clean_hex_str(hex);

    // 限制 4 个字符 (2 字节)
    // (我们无视 Java 奇怪的 4 字节限制, 采用正确的 2 字节)
    if v.len() > 4 {
        return Err(ProtocolError::HexLengthError {
            context: "i16",
            max_chars: 4,
            actual_chars: v.len(),
        });
    }
    if v.is_empty() {
        return Ok(0);
    }
    // 1. 解析为 u16
    let unsigned_val = u16::from_str_radix(v, 16).map_err(|e| ProtocolError::HexParseError {
        context: "i16 (from u16)",
        reason: e.to_string(),
    })?;
    // 2. 按位转换为 i16 (这 1:1 匹配了 Java 的 .shortValue() 行为)
    Ok(unsigned_val as i16)
}

/**
 * hex -> i32 (有符号 32-bit 整数)
 *
 * (例如: "FFFFFFFF" -> 4294967295 (u32) -> -1 (i32))
 */
pub fn hex_to_i32(hex: &str) -> ProtocolResult<i32> {
    ensure_is_machine_code(hex)?;
    let v = clean_hex_str(hex);
    // 限制 8 个字符 (4 字节)
    if v.len() > 8 {
        return Err(ProtocolError::HexLengthError {
            context: "i32",
            max_chars: 8,
            actual_chars: v.len(),
        });
    }
    if v.is_empty() {
        return Ok(0);
    }
    // 1. 解析为 u32
    let unsigned_val = u32::from_str_radix(v, 16).map_err(|e| ProtocolError::HexParseError {
        context: "i32 (from u32)",
        reason: e.to_string(),
    })?;
    // 2. 按位转换为 i32 (这 1:1 匹配了 Java 的 .intValue() 行为)
    Ok(unsigned_val as i32)
}

pub fn i32_to_hex(number: i32, expected_byte_length: usize) -> ProtocolResult<String> {
    // 1. 获取 i32 的标准 32-bit (4 字节, 8 字符) 的比特表示
    //    `number as u32` 是获取比特位的地道方式
    //    - 22 (i32)  -> 0x00000016 (u32)
    //    - -1 (i32)  -> 0xFFFFFFFF (u32)
    //    - 256 (i32) -> 0x00000100 (u32)
    let native_hex = format!("{:08X}", number as u32);

    // 2. 获取期望的字符长度
    let expected_char_length = expected_byte_length * 2;
    const NATIVE_CHAR_LENGTH: usize = 8; // i32 是 8 字符

    match expected_char_length.cmp(&NATIVE_CHAR_LENGTH) {
        // --- 截断 (Fixes Java Bug) ---
        // 期望 2 字节 (4 chars) < 本地 4 字节 (8 chars)
        std::cmp::Ordering::Less => {
            // (截断)
            let start_index = NATIVE_CHAR_LENGTH - expected_char_length;
            Ok(native_hex[start_index..].to_string())
        }

        // --- 长度相等 ---
        // 期望 4 字节 (8 chars) == 本地 4 字节 (8 chars)
        std::cmp::Ordering::Equal => {
            // (长度相等)
            Ok(native_hex)
        }

        // --- 补位 (Sign Extension) ---
        // 期望 8 字节 (16 chars) > 本地 4 字节 (8 chars)
        std::cmp::Ordering::Greater => {
            // (补位)
            // 我们需要根据符号位决定补 '0' 还是 'F'
            let padding_char = if number < 0 { 'F' } else { '0' };
            let padding_len = expected_char_length - NATIVE_CHAR_LENGTH;

            let mut padded_hex = String::with_capacity(expected_char_length);

            // 1. 填充 0 或 F
            for _ in 0..padding_len {
                padded_hex.push(padding_char);
            }
            // 2. 附加原始的 hex
            padded_hex.push_str(&native_hex);

            Ok(padded_hex)
        }
    }
}

/**
 * i16 (有符号 16-bit) -> hex-string (大写, 带补位或截断)
 */
pub fn i16_to_hex(number: i16, expected_byte_length: usize) -> ProtocolResult<String> {
    // 逻辑与 i32 版本完全相同, 只是本地长度变成了 4
    let native_hex = format!("{:04X}", number as u16);

    let expected_char_length = expected_byte_length * 2;
    const NATIVE_CHAR_LENGTH: usize = 4; // i16 是 4 字符

    match expected_char_length.cmp(&NATIVE_CHAR_LENGTH) {
        std::cmp::Ordering::Less => {
            // 截断 (例如, 2 字节的 i16 -> 1 字节)
            let start_index = NATIVE_CHAR_LENGTH - expected_char_length;
            Ok(native_hex[start_index..].to_string())
        }
        std::cmp::Ordering::Equal => Ok(native_hex),
        std::cmp::Ordering::Greater => {
            // 补位 (例如, 2 字节的 i16 -> 4 字节)
            let padding_char = if number < 0 { 'F' } else { '0' };
            let padding_len = expected_char_length - NATIVE_CHAR_LENGTH;

            let mut padded_hex = String::with_capacity(expected_char_length);
            for _ in 0..padding_len {
                padded_hex.push(padding_char);
            }
            padded_hex.push_str(&native_hex);
            Ok(padded_hex)
        }
    }
}

/**
 * u32 (无符号 32-bit) -> hex-string (大写, 带补位或截断)
 *
 * 补位总是使用 '0' (零扩展)。
 */
pub fn u32_to_hex(number: u32, expected_byte_length: usize) -> ProtocolResult<String> {
    let native_hex = format!("{:08X}", number);

    let expected_char_length = expected_byte_length * 2;
    const NATIVE_CHAR_LENGTH: usize = 8; // u32 是 8 字符

    match expected_char_length.cmp(&NATIVE_CHAR_LENGTH) {
        // --- 截断 ---
        std::cmp::Ordering::Less => {
            // 截取低位 (右侧)
            let start_index = NATIVE_CHAR_LENGTH - expected_char_length;
            Ok(native_hex[start_index..].to_string())
        }

        // --- 长度相等 ---
        std::cmp::Ordering::Equal => Ok(native_hex),

        // --- 补位 (零扩展) ---
        std::cmp::Ordering::Greater => {
            let padding_len = expected_char_length - NATIVE_CHAR_LENGTH;

            // `uN` 类型总是补 '0'
            let mut padded_hex = String::with_capacity(expected_char_length);
            for _ in 0..padding_len {
                padded_hex.push('0');
            }
            padded_hex.push_str(&native_hex);

            Ok(padded_hex)
        }
    }
}

/**
 * u16 (无符号 16-bit) -> hex-string (大写, 带补位或截断)
 *
 * 补位总是使用 '0' (零扩展)。
 */
pub fn u16_to_hex(number: u16, expected_byte_length: usize) -> ProtocolResult<String> {
    let native_hex = format!("{:04X}", number);

    let expected_char_length = expected_byte_length * 2;
    const NATIVE_CHAR_LENGTH: usize = 4; // u16 是 4 字符

    match expected_char_length.cmp(&NATIVE_CHAR_LENGTH) {
        std::cmp::Ordering::Less => {
            // 截断
            let start_index = NATIVE_CHAR_LENGTH - expected_char_length;
            Ok(native_hex[start_index..].to_string())
        }
        std::cmp::Ordering::Equal => Ok(native_hex),
        std::cmp::Ordering::Greater => {
            // 补位 (零扩展)
            let padding_len = expected_char_length - NATIVE_CHAR_LENGTH;
            let mut padded_hex = String::with_capacity(expected_char_length);
            for _ in 0..padding_len {
                padded_hex.push('0');
            }
            padded_hex.push_str(&native_hex);
            Ok(padded_hex)
        }
    }
}

/** i8 -> 8-bit binary-string */
pub fn i8_to_binary_str(number: i8) -> ProtocolResult<String> {
    Ok(format!("{:08b}", number as u8))
}

/** u8 -> 8-bit binary-string */
pub fn u8_to_binary_str(number: u8) -> ProtocolResult<String> {
    Ok(format!("{:08b}", number))
}

/**
 * 核心辅助函数：正确实现比特的零扩展或截断
 *
 * @param number_bits    要格式化的比特位 (u64足以容纳所有类型)
 * @param native_width   原始类型的宽度 (例如 8, 16, 32, 64)
 * @param expected_bit_length 期望的输出长度
 */
fn number_to_bits(
    number_bits: u64,
    native_width: u32,
    expected_bit_length: usize,
) -> ProtocolResult<String> {
    if expected_bit_length == 0 {
        return Err(ProtocolError::BinaryLengthErrorNegative { bits: 0 });
    }

    // 1. 获取 *完整宽度* 的本地二进制字符串
    //    例如: format_bits(10, 32, 16)
    //    native_binary = "00000000000000000000000000001010"
    let native_binary = format!("{number_bits:0>width$b}", width = native_width as usize);
    let native_len = native_width as usize;

    match expected_bit_length.cmp(&native_len) {
        // --- 截断 (Truncation) ---
        // 期望 16 < 本地 32
        std::cmp::Ordering::Less => {
            let start_index = native_len - expected_bit_length;
            // 示例: [32-16..] -> [16..]
            // "0...0101011010001000"[16..] -> "0101011010001000" (正确!)
            Ok(native_binary[start_index..].to_string())
        }
        // --- 长度相等 ---
        std::cmp::Ordering::Equal => Ok(native_binary),
        // --- 补位 (Zero-Padding) ---
        // 期望 40 > 本地 32
        std::cmp::Ordering::Greater => {
            let padding_len = expected_bit_length - native_len;
            let mut padded_binary = String::with_capacity(expected_bit_length);
            // 总是补 0 (零扩展)
            for _ in 0..padding_len {
                padded_binary.push('0');
            }
            padded_binary.push_str(&native_binary);
            Ok(padded_binary)
        }
    }
}

pub fn i32_to_binary_str(number: i32, expected_bit_length: usize) -> ProtocolResult<String> {
    // 将 i32 的比特位 (u32) 传入
    number_to_bits(number as u32 as u64, 32, expected_bit_length)
}

pub fn u32_to_binary_str(number: u32, expected_bit_length: usize) -> ProtocolResult<String> {
    number_to_bits(number as u64, 32, expected_bit_length)
}

pub fn i16_to_binary_str(number: i16, expected_bit_length: usize) -> ProtocolResult<String> {
    number_to_bits(number as u16 as u64, 16, expected_bit_length)
}

pub fn u16_to_binary_str(number: u16, expected_bit_length: usize) -> ProtocolResult<String> {
    number_to_bits(number as u64, 16, expected_bit_length)
}

/**
 * binary-string -> i32 (有符号 32-bit)
 *
 * 这将正确地将 "11111111111111111111111111111111" (32 bits) 解析为 -1 (i32)
 */
pub fn binary_str_to_i32(binary_str: &str) -> ProtocolResult<i32> {
    ensure_is_machine_code(binary_str)?;
    // 1. 将字符串按 radix 2 (二进制) 解析为 u32
    let unsigned_val =
        u32::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
            context: "i32 (from u32)",
            reason: e.to_string(),
        })?;

    // 2. 将 u32 的比特位重新解释为 i32
    Ok(unsigned_val as i32)
}

/**
 * binary-string -> u32 (无符号 32-bit)
 */
pub fn binary_str_to_u32(binary_str: &str) -> ProtocolResult<u32> {
    ensure_is_machine_code(binary_str)?;
    u32::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
        context: "u32",
        reason: e.to_string(),
    })
}

/**
 * binary-string -> i16 (有符号 16-bit)
 */
pub fn binary_str_to_i16(binary_str: &str) -> ProtocolResult<i16> {
    ensure_is_machine_code(binary_str)?;
    let unsigned_val =
        u16::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
            context: "i16 (from u16)",
            reason: e.to_string(),
        })?;
    Ok(unsigned_val as i16)
}

/**
 * binary-string -> u16 (无符号 16-bit)
 */
pub fn binary_str_to_u16(binary_str: &str) -> ProtocolResult<u16> {
    ensure_is_machine_code(binary_str)?;
    u16::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
        context: "u16",
        reason: e.to_string(),
    })
}

/**
 * binary-string -> i8 (有符号 8-bit)
 */
pub fn binary_str_to_i8(binary_str: &str) -> ProtocolResult<i8> {
    ensure_is_machine_code(binary_str)?;
    let unsigned_val =
        u8::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
            context: "i8 (from u8)",
            reason: e.to_string(),
        })?;
    Ok(unsigned_val as i8)
}

/**
 * binary-string -> u8 (无符号 8-bit)
 */
pub fn binary_str_to_u8(binary_str: &str) -> ProtocolResult<u8> {
    ensure_is_machine_code(binary_str)?;
    u8::from_str_radix(binary_str, 2).map_err(|e| ProtocolError::BinaryParseError {
        context: "u8",
        reason: e.to_string(),
    })
}

pub fn binary_str_to_bits(binary_str: &str) -> ProtocolResult<Vec<bool>> {
    ensure_is_machine_code(binary_str)?;
    binary_str
        .chars()
        .map(|c| match c {
            '1' => Ok(true),
            '0' => Ok(false),
            // 如果是无效字符, 返回 Err(c)
            invalid_char => Err(invalid_char),
        })
        // 1. 如果所有都是 Ok(T), 它返回 Ok(Vec<T>)
        // 2. 如果遇到 *第一个* Err(E), 它会立即停止并返回 Err(E)
        .collect::<Result<Vec<bool>, char>>()
        .map_err(|invalid_char| ProtocolError::BinaryParseError {
            context: "Vec<bool>",
            reason: format!(
                "Invalid character '{}' found in binary string",
                invalid_char
            ),
        })
}

pub fn swap(hex: &str) -> ProtocolResult<String> {
    let mut bytes = hex_to_bytes(hex)?;
    bytes.reverse();
    bytes_to_hex(bytes.as_slice())
}

pub fn swap_bytes(bytes: &[u8]) -> ProtocolResult<Vec<u8>> {
    let mut new_bytes = Vec::with_capacity(bytes.len());
    for byte in bytes {
        new_bytes.push(byte.reverse_bits());
    }
    Ok(new_bytes)
}

/**
 * 对一个给定的 byte 数组切片按照索引截取 (返回一个新的 Vec<u8>)。
 *
 * - 索引为负数: 从末尾计算。
 * - `end_index == 0`: (且非负时) 截取到末尾。
 * - 范围无效 (start >= end): 返回空 Vec (panic-safe)。
 * - 范围越界: 自动截断 (clamp) 到 0..data.len() (panic-safe)。
 */
pub fn cut_bytes(data: &[u8], start_index: i64, end_index: i64) -> ProtocolResult<Vec<u8>> {
    let total_length = data.len();
    let total_length_i64 = total_length as i64;

    if start_index == 0 && end_index == 0 {
        return Ok(data.to_vec());
    }

    // 2. 两个脚标都为负数的时候，startIndex必须大于endIndex
    if start_index < 0 && end_index < 0 && start_index > end_index {
        return Err(ProtocolError::InvalidRange {
            start: start_index,
            end: end_index,
            reason: "When both indices are negative, start_index must be <= end_index".to_string(),
        });
    }

    // 3. 将 start_index (i64) 解析为 final_start (usize)
    let final_start = if start_index < 0 {
        // 负数索引: 从末尾计算, 并确保不小于 0
        (total_length_i64 + start_index).max(0) as usize
    } else {
        // 正数索引
        (start_index as usize).min(total_length)
    };

    // 4. 将 end_index (i64) 解析为 final_end (usize)
    let final_end = if end_index < 0 {
        // 负数索引: 从末尾计算, 并确保不小于 0
        (total_length_i64 + end_index).max(0) as usize
    } else if end_index == 0 {
        // 快捷方式: 截取到末尾
        total_length
    } else {
        // 正数索引
        (end_index as usize).min(total_length)
    };

    // 5. 执行 panic-safe 的切片
    //    如果 `final_start >= final_end` (例如 9..7, 或 9..9),
    //    `data.get()` 会返回 `None`, 将其映射为空 Vec,
    let result_slice = data.get(final_start..final_end).unwrap_or(&[]); // 如果范围无效, 返回空切片

    Ok(result_slice.to_vec())
}

pub fn cut_hex(hex: &str, start_index: i64, end_index: i64) -> ProtocolResult<Vec<u8>> {
    let bytes = hex_to_bytes(hex)?;
    cut_bytes(bytes.as_slice(), start_index, end_index)
}

/**
 * 检查字符串是否为有效的 BCD 码
 */
pub fn is_bcd(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

/**
 * 检查字符串是否为有效的 Hex 码 (偶数长度, 0-9, a-f, A-F)
 */
pub fn is_hex(s: &str) -> bool {
    hex::decode(s).is_ok()
}

/**
 * 检查字符串是否为有效的 ASCII (Hex) 码
 * (偶数长度, 0-9, a-f, A-F, 且每个字节值 <= 127)
 */
pub fn is_ascii_hex(s: &str) -> bool {
    // 1. 尝试将 hex 解码为字节
    let bytes = match hex::decode(s) {
        Ok(b) => b,
        // 如果不是有效 hex (奇数长度或非法字符), 则不是 ascii-hex
        Err(_) => return false,
    };

    // 2. 检查所有字节是否都在 ASCII 范围 (0-127)
    bytes.iter().all(|b| b.is_ascii()) // b.is_ascii() 检查 *b <= 127
}

pub fn ensure_is_machine_code(s: &str) -> ProtocolResult<()> {
    let fail = is_hex(s) || is_ascii_hex(s) || is_bcd(s);
    match fail {
        true => Ok(()),
        false => Err(ProtocolError::NotMachineCode(s.into())),
    }
}

pub fn ensure_is_bcd(s: &str) -> ProtocolResult<()> {
    if is_bcd(s) {
        Ok(())
    } else {
        Err(ProtocolError::NotBcd(s.into()))
    }
}

pub fn ensure_is_ascii_hex(s: &str) -> ProtocolResult<()> {
    if is_ascii_hex(s) {
        Ok(())
    } else {
        Err(ProtocolError::NotAscii(s.into()))
    }
}

pub fn ascii_to_string(ascii_hex_str: &str) -> ProtocolResult<String> {
    if ascii_hex_str.is_empty() {
        return Ok(String::new());
    }

    // 1. 清理 "0x" 前缀
    let v = clean_hex_str(ascii_hex_str);

    // 2. 验证
    ensure_is_ascii_hex(v)?;

    // 3. 转换
    let bytes = hex::decode(v).unwrap();

    // 4. 将字节转换为 String
    Ok(String::from_utf8(bytes).unwrap())
}

pub fn string_to_ascii(plain_str: &str) -> ProtocolResult<String> {
    if plain_str.is_empty() {
        return Ok(String::new());
    }

    // 1. 验证输入是否为纯 ASCII
    if !plain_str.is_ascii() {
        return Err(ProtocolError::NotAscii(
            "Input string contains non-ASCII characters".into(),
        ));
    }

    // 2. 将 ASCII 字符串的字节转换为 Hex
    bytes_to_hex(plain_str.as_bytes())
}

/**
 * 替换 byte 数组中的某一段 (返回一个新的 Vec<u8>)
 *
 * @param ori_bytes         原文 byte 切片
 * @param start_byte_pos    替换起始位置 (包) (必须为 0 或正数)
 * @param end_byte_pos      替换终止位置 (不包) (如果为负值,就倒着数)
 * @param replace_bytes     要替换的 byte 切片
 * @return 替换后的 Vec<u8>
 */
pub fn replace_bytes(
    ori_bytes: &[u8],
    start_byte_pos: i64,
    end_byte_pos: i64,
    replace_bytes: &[u8],
) -> ProtocolResult<Vec<u8>> {
    // 非空校验
    if ori_bytes.is_empty() {
        return Err(ProtocolError::InvalidInput(
            "Original bytes cannot be empty".into(),
        ));
    }
    if replace_bytes.is_empty() {
        return Err(ProtocolError::InvalidInput(
            "Replacement bytes cannot be empty".into(),
        ));
    }

    let total_length = ori_bytes.len();
    let total_length_i64 = total_length as i64;

    // start_byte_pos 校验
    if start_byte_pos < 0 {
        return Err(ProtocolError::InvalidRange {
            start: start_byte_pos,
            end: end_byte_pos,
            reason: "start_byte_pos must be 0 or positive".into(),
        });
    }

    // 当 end_byte_pos 为正时
    if end_byte_pos > 0 {
        if start_byte_pos > end_byte_pos {
            return Err(ProtocolError::InvalidRange {
                start: start_byte_pos,
                end: end_byte_pos,
                reason: "start_byte_pos must be less than or equal to end_byte_pos".into(),
            });
        }
        if end_byte_pos > total_length_i64 {
            return Err(ProtocolError::InvalidRange {
                start: start_byte_pos,
                end: end_byte_pos,
                reason: "end_byte_pos must be less than or equal to original bytes length".into(),
            });
        }
    }

    // 1. 解析起始索引
    let final_start = (start_byte_pos as usize).min(total_length);

    // 2. 解析终止索引 (ebp)
    // endBytePos 逻辑
    let final_end = if end_byte_pos > 0 {
        (end_byte_pos as usize).min(total_length)
    } else {
        // 负值或 0: 从末尾计算, 并确保不小于 0
        (total_length_i64 + end_byte_pos).max(0) as usize
    };

    // 3. 最终范围校验 (防止 panic)
    if final_start > final_end {
        return Err(ProtocolError::InvalidRange {
            start: start_byte_pos,
            end: end_byte_pos,
            reason: "Resolved start index is greater than resolved end index".into(),
        });
    }
    // (final_end > total_length 的情况已被上面的校验覆盖)

    // a. 复制原始 bytes, 因为 `splice` 需要一个可变的 `Vec`
    let mut result_vec = ori_bytes.to_vec();

    // b. `splice` 会在 `final_start..final_end` 范围内
    //    移除所有元素, 并插入 `replace_bytes`
    result_vec.splice(final_start..final_end, replace_bytes.iter().copied());

    Ok(result_vec)
}

/**
 * 替换 hex-string 字节中的某一段
 *
 * @param ori_hex         原文 hex-string
 * @param start_byte_pos  替换起始字节位置 (包)
 * @param end_byte_pos    替换终止字节位置 (不包) (如果为负值,就倒着数)
 * @param dest_hex        要替换的 hex-string
 * @return 替换后的 hex-string
 */
pub fn replace_hex(
    ori_hex: &str,
    start_byte_pos: i64,
    end_byte_pos: i64,
    dest_hex: &str,
) -> ProtocolResult<String> {
    // 1. Hex -> Bytes
    let ori_bytes = hex_to_bytes(ori_hex)?;
    let dest_bytes = hex_to_bytes(dest_hex)?;
    // 2. 调用 bytes_util 中的核心逻辑
    let result_bytes = replace_bytes(&ori_bytes, start_byte_pos, end_byte_pos, &dest_bytes)?;
    // 3. Bytes -> Hex
    bytes_to_hex(&result_bytes)
}

/**
 * 按块大小 (block size) 补位 (返回一个新的 Vec<u8>)
 *
 * - len == block_size -> 不补位
 * - len < block_size  -> 补 (block_size - len)
 * - len > block_size  -> 补至下一个 `block_size` 的倍数 (包括在 len 是倍数时补一个完整块)
 *
 * @param data          原文 byte 切片
 * @param block_size    补位的基准长度 (digit)
 * @param padding_byte  补位字节。如果为 `None` (对应 Java `ph` 为空),
 * 则使用 PKCS#7 风格, 补位字节的值 = 补位的长度。
 * @return 补位之后的 Vec<u8>
 */
pub fn pad_bytes_to_block_size(
    data: &[u8],
    block_size: usize,
    padding_byte: Option<u8>,
) -> ProtocolResult<Vec<u8>> {
    let origin_length = data.len();

    if block_size == 0 {
        return Err(ProtocolError::InvalidInput(
            "Block size (digit) must be positive".into(),
        ));
    }

    // --- 1:1 翻译 Java 的补位长度 (short_by) 计算 ---
    let short_by = if origin_length == block_size {
        // Java: `short_by` 默认为 0, `if (short_by == 0)` return
        0
    } else if origin_length < block_size {
        // Java: `short_by = digit - origin_length`
        block_size - origin_length
    } else {
        // Java: `origin_length > digit`
        // `short_by = digit - (origin_length % digit)`
        // 这包括了 `origin_length % digit == 0` 时, short_by = digit (补一个整块)
        let remainder = origin_length % block_size;
        if remainder == 0 {
            block_size
        } else {
            block_size - remainder
        }
    };
    // --- 结束 1:1 翻译 ---

    // `short_by == 0` 对应 Java 的 `return hex`
    if short_by == 0 {
        return Ok(data.to_vec());
    }

    // 确定补位字节 (pad_val)
    let pad_val = match padding_byte {
        Some(b) => b,
        // 对应 Java: `ph = HexUtil.int2Hex(short_by, 1)`
        None => {
            if short_by > 255 {
                // PKCS#7 补位值不能超过 255
                return Err(ProtocolError::InvalidInput(format!(
                    "Default PKCS#7 padding length ({}) exceeds 255",
                    short_by
                )));
            }
            short_by as u8
        }
    };

    let new_len = origin_length + short_by;
    let mut result_vec = Vec::with_capacity(new_len);
    result_vec.extend_from_slice(data);
    // `resize` 是 Rust 中最地道的补位方法
    result_vec.resize(new_len, pad_val);

    Ok(result_vec)
}

/**
 * 补位到指定的总字节长度 (返回一个新的 Vec<u8>)
 *
 * @param data            原文 byte 切片
 * @param total_length    期望补位完成后的总长度
 * @param append_on_tail  是否补位在末尾 (true=末尾, false=开头)
 * @param padding_byte    补位字节。如果为 `None`, 使用 PKCS#7 风格。
 * @return 补位完成后的 Vec<u8>
 */
pub fn pad_bytes_to_length(
    data: &[u8],
    total_length: usize,
    append_on_tail: bool,
    padding_byte: Option<u8>,
) -> ProtocolResult<Vec<u8>> {
    let origin_length = data.len();

    if origin_length > total_length {
        return Err(ProtocolError::PaddingError {
            original_len: origin_length,
            target_len: total_length,
        });
    }

    let short_by = total_length - origin_length;
    if short_by == 0 {
        return Ok(data.to_vec());
    }

    // 确定补位字节 (pad_val)
    let pad_val = match padding_byte {
        Some(b) => b,
        // 对应 Java: `appendHex = HexUtil.int2Hex(short_by, 1)`
        None => {
            if short_by > 255 {
                return Err(ProtocolError::InvalidInput(format!(
                    "Default PKCS#7 padding length ({}) exceeds 255",
                    short_by
                )));
            }
            short_by as u8
        }
    };

    let mut result_vec = Vec::with_capacity(total_length);

    if append_on_tail {
        // 补在末尾
        result_vec.extend_from_slice(data);
        result_vec.resize(total_length, pad_val);
    } else {
        // 补在开头
        result_vec.resize(short_by, pad_val);
        result_vec.extend_from_slice(data);
    }

    Ok(result_vec)
}

fn parse_padding_hex(padding_hex: Option<&str>) -> ProtocolResult<Option<u8>> {
    match padding_hex {
        None | Some("") | Some(..) if padding_hex.unwrap_or("").trim().is_empty() => Ok(None),
        // `ph` 非空
        Some(ph_str) => {
            let pad_bytes = hex_to_bytes(ph_str.trim())?;
            // 补位 hex 必须是 1 字节
            if pad_bytes.len() != 1 {
                Err(ProtocolError::InvalidInput(format!(
                    "Padding hex must be exactly 1 byte (2 chars), but got: {}",
                    ph_str
                )))
            } else {
                Ok(Some(pad_bytes[0]))
            }
        }
        None => Err(ProtocolError::PaddingError {
            original_len: 0,
            target_len: 0,
        }),
    }
}

// --- 本次新增的函数 ---

/**
 * 按块大小 (block size) 补位 hex 字符串
 */
pub fn pad_hex_to_block_size(
    hex: &str,
    block_size: usize,
    padding_hex: Option<&str>,
) -> ProtocolResult<String> {
    // 1. Hex -> Bytes (健壮性)
    let data = hex_to_bytes(hex)?;

    // 2. 解析 padding_hex (e.g., "00") 为 padding_byte (e.g., 0x00)
    let padding_byte = parse_padding_hex(padding_hex)?;

    // 3. 调用核心字节逻辑
    let padded_bytes = pad_bytes_to_block_size(&data, block_size, padding_byte)?;

    // 4. Bytes -> Hex
    bytes_to_hex(&padded_bytes)
}

/**
 * 补位 hex 字符串到指定的总字节长度
 */
pub fn pad_hex_to_length(
    hex: &str,
    total_length: usize,
    append_on_tail: bool,
    padding_hex: Option<&str>,
) -> ProtocolResult<String> {
    // 1. Hex -> Bytes (健壮性)
    let data = hex_to_bytes(hex)?;

    // 2. 解析 padding_hex (e.g., "00") 为 padding_byte (e.g., 0x00)
    let padding_byte = parse_padding_hex(padding_hex)?;

    // 3. 调用核心字节逻辑
    let padded_bytes = pad_bytes_to_length(&data, total_length, append_on_tail, padding_byte)?;

    // 4. Bytes -> Hex
    bytes_to_hex(&padded_bytes)
}
