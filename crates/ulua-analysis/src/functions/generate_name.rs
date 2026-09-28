extern crate alloc;

use alloc::string::{String, ToString};

/// 生成名字前的最大搜索尝试次数（cpp `ToString.cpp` 中的字面量 256）。
pub const MAX_GENERATED_NAME_ATTEMPTS: usize = 256;

/// cpp `ToString.cpp` 的 `kGenericTypeLetters`：泛型命名字母池。
/// 首选 T/U/V/W/X/Y/Z，其后回落 A..S（i%26 索引）。
const GENERIC_TYPE_LETTERS: &[u8; 26] = b"TUVWXYZABCDEFGHIJKLMNOPQRS";
/// 字母池长度（cpp `kGenericTypeLetters` 的 26 项取模域）。
const K_LETTER_COUNT: usize = 26;

/// C++ `std::string generateName(size_t i, bool isForGeneric)`（ToString.cpp）。
pub fn generate_name(i: usize, is_for_generic: bool) -> String {
  let mut n = String::new();
  if is_for_generic {
    n.push(GENERIC_TYPE_LETTERS[i % K_LETTER_COUNT] as char);
  } else {
    n.push((b'a' + (i % K_LETTER_COUNT) as u8) as char);
  }
  if i >= K_LETTER_COUNT {
    n += &(i / K_LETTER_COUNT).to_string();
  }
  n
}
