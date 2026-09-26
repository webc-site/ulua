use alloc::string::String;

/// C++ escape 的预分配余量（C++ 注释：arbitrary number to guess how many
/// characters we'll be inserting）。
const RESERVE_SLACK: usize = 50;

/// cpp `escape` default 分支 `formatAppend(r, "%03u", c)` 的三位定宽十进制
/// 数字（ASCII）。`&str` 版 [`escape`] 与 `ulua-ast` 的字节版 `escape_bytes`
/// 共用，避免两处各写一遍除法；`c` 为 `u8`，最大 255 恒三位，无溢出。
pub fn decimal_escape_digits(c: u8) -> [u8; 3] {
  [b'0' + c / 100, b'0' + (c % 100) / 10, b'0' + c % 10]
}

/// `Luau::escape`：转义控制字符与特殊符号。参考：`Common/src/StringUtils.cpp`。
///
/// 逐字符 FSM：可打印且非特殊符号的字符原样透传（与 C++ 的 `r += c` 一致——
/// 非 ASCII 字符即多字节 UTF-8 序列，按字符写回与按字节写回产出同一字节串），
/// 其余替换为反斜杠转义。全程在 `String` 上构造，因此输出恒为合法 UTF-8，
/// 不需要 `from_utf8_unchecked`。
pub fn escape(s: &str, escape_for_interp_string: bool) -> String {
  let mut r = String::with_capacity(s.len() + RESERVE_SLACK);

  for c in s.chars() {
    if !c.is_ascii() || (c >= ' ' && !matches!(c, '\\' | '\'' | '"' | '`' | '{')) {
      r.push(c);
      continue;
    }

    r.push('\\');

    if escape_for_interp_string && matches!(c, '`' | '{') {
      r.push(c);
      continue;
    }

    // C++ switch 的转义映射（\a \b \f \n \r \t \v 与引号、反斜杠）。
    match c {
      '\u{7}' => r.push('a'),
      '\u{8}' => r.push('b'),
      '\u{c}' => r.push('f'),
      '\n' => r.push('n'),
      '\r' => r.push('r'),
      '\t' => r.push('t'),
      '\u{b}' => r.push('v'),
      '\'' | '"' | '\\' => r.push(c),
      // 至此只剩 ASCII 控制字符（`c < ' '`），`c as u8` 无损。
      _ => r.extend(decimal_escape_digits(c as u8).map(char::from)),
    }
  }

  r
}
