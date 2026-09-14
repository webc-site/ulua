/// 镜像 C 的 `atoi(s)`: 跳过前缀空白与符号, 解析数字前缀, 忽略尾部字符;
/// 非数字输入得 0, 溢出 clamp 到 i32 范围 (cpp atoi 溢出为 UB, 这里取安全语义)
pub fn atoi(value: &str) -> i32 {
  let bytes = value.as_bytes();
  let mut idx = 0;

  while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
    idx += 1;
  }

  let mut sign = 1i64;
  if idx < bytes.len() {
    if bytes[idx] == b'-' {
      sign = -1;
      idx += 1;
    } else if bytes[idx] == b'+' {
      idx += 1;
    }
  }

  let mut result = 0i64;
  while idx < bytes.len() && bytes[idx].is_ascii_digit() {
    result = result
      .saturating_mul(10)
      .saturating_add((bytes[idx] - b'0') as i64);
    idx += 1;
  }

  (result.saturating_mul(sign)).clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
