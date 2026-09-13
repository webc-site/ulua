use alloc::{string::String, vec::Vec};

/// C++ escape 的预分配余量（C++ 注释：arbitrary number to guess how many
/// characters we'll be inserting）。
const RESERVE_SLACK: usize = 50;

/// `Luau::escape`：转义控制字符与特殊符号。参考：`Common/src/StringUtils.cpp`。
///
/// 逐字节 FSM：`c >= b' '` 且非特殊符号的原字节直接透传（与 C++ 的
/// `r += c` 一致，多字节 UTF-8 序列保持不变）。
pub fn escape(s: &str, escape_for_interp_string: bool) -> String {
  let mut r = Vec::with_capacity(s.len() + RESERVE_SLACK);

  for &c in s.as_bytes() {
    if c >= b' ' && c != b'\\' && c != b'\'' && c != b'\"' && c != b'`' && c != b'{' {
      r.push(c);
    } else {
      r.push(b'\\');

      if escape_for_interp_string && (c == b'`' || c == b'{') {
        r.push(c);
        continue;
      }

      // C++ switch 的转义映射（\a \b \f \n \r \t \v 与引号、反斜杠）。
      match c {
        0x07 => r.push(b'a'),
        0x08 => r.push(b'b'),
        0x0C => r.push(b'f'),
        b'\n' => r.push(b'n'),
        b'\r' => r.push(b'r'),
        b'\t' => r.push(b't'),
        0x0B => r.push(b'v'),
        b'\'' => r.push(b'\''),
        b'\"' => r.push(b'\"'),
        b'\\' => r.push(b'\\'),
        // 上游用 `%03u`（十进制、零填充、宽度 3）输出，此处直接写数字。
        _ => {
          r.push(b'0' + c / 100);
          r.push(b'0' + (c % 100) / 10);
          r.push(b'0' + c % 10);
        }
      }
    }
  }

  // SAFETY：输入是合法 UTF-8，透传字节保持原序（多字节序列不被拆散），
  // 替换只引入 ASCII 字节，输出必为合法 UTF-8。
  unsafe { String::from_utf8_unchecked(r) }
}
