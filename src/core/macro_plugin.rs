// 内部辅助宏，用于简化整数类型的转换和缩放逻辑
#[macro_export]
macro_rules! handle_int {
    ($type:ty, $len:expr, $bytes:expr, $scale:expr) => {{
        // 1. 检查长度
        if $bytes.len() != $len {
            return Err(ProtocolError::ValidationFailed(format!(
                "Invalid byte length for {}. Expected {}, got {}",
                stringify!($type),
                $len,
                $bytes.len()
            )));
        }
        // 2. 从大端字节转换
        let value = <$type>::from_be_bytes($bytes.try_into().unwrap());
        // 3. 转换为f64，准备缩放
        let value_f64 = value as f64;
        // 4. 执行缩放 (如果需要)
        if $scale != 1.0 && $scale != 0.0 {
            // 假设 scale=1.0 表示不缩放
            // 使用您提供的高精度除法，假设默认精度和小数位数
            let scaled_value =
                math_util::multiply(6, DecimalRoundingMode::HalfUp, &[value_f64, $scale])?;
            Ok(scaled_value.to_string())
        } else if $scale == 0.0 {
            Err(ProtocolError::ValidationFailed(
                "Scale factor cannot be zero.".to_string(),
            ))
        } else {
            // 不缩放，直接转字符串
            Ok(value.to_string())
        }
    }};
}
