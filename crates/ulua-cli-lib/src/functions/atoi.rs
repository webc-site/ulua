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

  // 解析数字前缀：`take_while` 天然在首个非数字处停止（cpp 的 atoi 也是前缀语义），
  // `fold` 一步累加，饱和乘加代替手写循环里的 break。
  let result = digits
    .iter()
    .copied()
    .take_while(u8::is_ascii_digit)
    .fold(0i64, |acc, b| {
      acc.saturating_mul(10).saturating_add(i64::from(b - b'0'))
    });

  result
    .saturating_mul(sign)
    .clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
