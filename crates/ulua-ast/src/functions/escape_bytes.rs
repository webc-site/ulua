use alloc::vec::Vec;

use ulua_common::functions::escape::decimal_escape_digits;

/// C++ escape 的预分配余量（C++ 注释：arbitrary number to guess how many
/// characters we'll be inserting）。
const RESERVE_SLACK: usize = 50;

/// `Luau::escape` 的字节版（`Common/src/StringUtils.cpp` escape）。
///
/// 入参可为词法器 fixup 后的字符串值，含任意字节（`"\xff"` 非法 UTF-8），
/// 故不复用 `ulua_common::escape`（`&str` 签名，非法字节序列进不去）；两版
/// 共用的只有 [`decimal_escape_digits`] 这一处 `%03u` 数字计算。
/// 逐字节 FSM 与 cpp 完全一致：`c >= ' '` 且非特殊符号的原字节直接透传
/// （多字节 UTF-8 序列与非法字节均原样保留），控制字符按 `\a \b \f \n
/// \r \t \v`/引号/反斜杠映射，其余走 `%03u` 十进制转义。
pub(crate) fn escape_bytes(s: &[u8], escape_for_interp_string: bool) -> Vec<u8> {
  let mut r = Vec::with_capacity(s.len() + RESERVE_SLACK);

  for &c in s {
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
        b'\x07' => r.push(b'a'),
        b'\x08' => r.push(b'b'),
        b'\x0c' => r.push(b'f'),
        b'\n' => r.push(b'n'),
        b'\r' => r.push(b'r'),
        b'\t' => r.push(b't'),
        b'\x0b' => r.push(b'v'),
        b'\'' => r.push(b'\''),
        b'\"' => r.push(b'\"'),
        b'\\' => r.push(b'\\'),
        // `%03u`（十进制、零填充、宽度 3）的数字与 &str 版共用同一实现。
        _ => r.extend_from_slice(&decimal_escape_digits(c)),
      }
    }
  }

  r
}
