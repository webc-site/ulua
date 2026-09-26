pub fn safe_integer_constant(value: f64) -> bool {
  // 32 位范围内；既允许最大无符号数，也允许其负数对应值
  // double 实际可支持更大范围（但并非精确 2^53），不过本函数用于 32 位优化
  if value < -4294967295.0 || value > 4294967295.0 {
    return false;
  }

  (value as i64 as f64) == value
}
