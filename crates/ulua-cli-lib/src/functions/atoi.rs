/// 镜像 C 的 `atoi(s)`: 跳过前缀空白与符号, 解析数字前缀, 忽略尾部字符;
/// 非数字输入得 0, 溢出 clamp 到 i32 范围 (cpp atoi 溢出为 UB, 这里取安全语义)
pub fn atoi(value: &str) -> i32 {
  let bytes = value.as_bytes();

  // 跳过前缀空白
  let start = bytes
    .iter()
    .position(|b| !b.is_ascii_whitespace())
    .unwrap_or(bytes.len());
  let rest = &bytes[start..];

  // 可选符号
  let (sign, digits) = match rest {
    [b'-', d @ ..] => (-1i64, d),
    [b'+', d @ ..] => (1i64, d),
    _ => (1i64, rest),
  };

  // 解析数字前缀
  let mut result = 0i64;
  for b in digits {
    if !b.is_ascii_digit() {
      break;
    }
    result = result
      .saturating_mul(10)
      .saturating_add(i64::from(b - b'0'));
  }

  result
    .saturating_mul(sign)
    .clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
