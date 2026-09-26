/// 纯 safe 的 Rust 切片格式说明符扫描函数。
///
/// 从 `bytes`（紧跟 `%` 之后的字节切片）开始解析：
/// - 标志位（`FLAGS = b"-+ #0"`）：每个标志最多出现一次，总长不得超过 FLAGS.len()；
/// - 字段宽度（最多 2 位 ASCII 数字）；
/// - 精度（可选的 `.` 紧随最多 2 位 ASCII 数字）；
///
/// 若遇到多余数字或重复标志，返回 `Err(&'static str)`；
/// 成功则返回 `Ok(spec_len)`，其中 `bytes[..spec_len]` 为格式选项，`bytes[spec_len]` 为转换指示符（如 'd', 's' 等）。
#[inline]
pub fn scan_format_spec(bytes: &[u8]) -> Result<usize, &'static str> {
  const FLAGS: &[u8] = b"-+ #0";

  let mut p = bytes
    .iter()
    .take_while(|&&b| b != 0 && FLAGS.contains(&b))
    .count();

  // C++: (size_t)(p - strfrmt) >= sizeof(FLAGS), 其中 sizeof(FLAGS) 为 6 (包含 '\0')，即最多 5 个不同 flag
  if p > FLAGS.len() {
    return Err("invalid format (repeated flags)");
  }

  if p < bytes.len() && bytes[p].is_ascii_digit() {
    p += 1;
  }
  if p < bytes.len() && bytes[p].is_ascii_digit() {
    p += 1;
  }

  if p < bytes.len() && bytes[p] == b'.' {
    p += 1;
    if p < bytes.len() && bytes[p].is_ascii_digit() {
      p += 1;
    }
    if p < bytes.len() && bytes[p].is_ascii_digit() {
      p += 1;
    }
  }

  if p < bytes.len() && bytes[p].is_ascii_digit() {
    return Err("invalid format (width or precision too long)");
  }

  Ok(p)
}

/// 格式说明符扫描上限（字节）：flags(5) + width(2) + '.'(1) + prec(2) + indicator(1) + 溢出数字(1)。
/// cpp 无限定扫描，由标志/位数上限兜底；此处显式封顶，行为等价。（原 `scanformat`
/// 指针出参版已删：`str_format` 直接在本函数上按下标推进。）
pub(crate) const MAX_FORMAT_SPEC_SCAN: usize = 16;
