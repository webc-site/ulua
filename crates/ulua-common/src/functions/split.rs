extern crate alloc;

use alloc::vec::Vec;

/// `Luau::split`：按分隔符切分。参考：`Common/src/StringUtils.cpp`。
/// C++ `char` 是单字节，Rust `char` 可能是多字节，故按 `len_utf8` 跳过，
/// 单字节（ASCII）分隔符行为与 C++ 一致。
pub fn split(mut s: &str, delimiter: char) -> Vec<&str> {
  let mut result = Vec::new();

  while !s.is_empty() {
    if let Some(index) = s.find(delimiter) {
      result.push(&s[..index]);
      s = &s[index + delimiter.len_utf8()..];
    } else {
      result.push(s);
      break;
    }
  }

  result
}
