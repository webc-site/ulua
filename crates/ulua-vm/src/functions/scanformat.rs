/// 标志位集 `{'-','+',' ','#','0'}` 在折叠掩码中的位值。
pub(crate) const FLAG_LEFT: u8 = 1 << 0;
pub(crate) const FLAG_PLUS: u8 = 1 << 1;
pub(crate) const FLAG_SPACE: u8 = 1 << 2;
pub(crate) const FLAG_ALT: u8 = 1 << 3;
pub(crate) const FLAG_ZERO: u8 = 1 << 4;

/// 标志前缀单遍位掩码折叠：自 `bytes` 起逐字节读取，命中标志字符置对应位，
/// 遇非标志字节即止，返回 `(掩码, 消耗字节数)`。NUL 本就不是标志成员，原
/// `b != 0` 判定并入默认截断臂；重复标志在掩码中置位幂等，保留“该字节出现
/// 于前缀”的集合语义（连排重复次数的上限仍由扫描侧按消耗计数把关）。
/// 本函数是 `scan_format_spec` 与 `parse_format_spec` 共用的唯一标志真源。
#[inline]
pub(crate) const fn fold_format_flags(bytes: &[u8]) -> (u8, usize) {
  let mut mask = 0u8;
  let mut n = 0usize;
  while n < bytes.len() {
    match bytes[n] {
      b'-' => mask |= FLAG_LEFT,
      b'+' => mask |= FLAG_PLUS,
      b' ' => mask |= FLAG_SPACE,
      b'#' => mask |= FLAG_ALT,
      b'0' => mask |= FLAG_ZERO,
      _ => break,
    }
    n += 1;
  }
  (mask, n)
}

/// 纯 safe 的 Rust 切片格式说明符扫描函数。
///
/// 从 `bytes`（紧跟 `%` 之后的字节切片）开始解析：
/// - 标志位（`- + # 0`，经 `fold_format_flags` 单遍折叠）：每个标志最多出现一次，
///   总长不得超过标志数（5）；
/// - 字段宽度（最多 2 位 ASCII 数字）；
/// - 精度（可选的 `.` 紧随最多 2 位 ASCII 数字）；
///
/// 若遇到多余数字或重复标志，返回 `Err(&'static str)`；
/// 成功则返回 `Ok(spec_len)`，其中 `bytes[..spec_len]` 为格式选项，`bytes[spec_len]` 为转换指示符（如 'd', 's' 等）。
#[inline]
pub fn scan_format_spec(bytes: &[u8]) -> Result<usize, &'static str> {
  // 不同标志个数（`-+ #0`）
  const FORMAT_FLAG_COUNT: usize = 5;

  let (_, mut p) = fold_format_flags(bytes);

  // C++: (size_t)(p - strfrmt) >= sizeof(FLAGS), 其中 sizeof(FLAGS) 为 6 (包含 '\0')，即最多 5 个不同 flag
  if p > FORMAT_FLAG_COUNT {
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
