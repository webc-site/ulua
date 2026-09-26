extern crate alloc;

use alloc::string::{String, ToString};

/// 生成名字前的最大搜索尝试次数（cpp `ToString.cpp` 中的字面量 256）。
pub const MAX_GENERATED_NAME_ATTEMPTS: usize = 256;

pub fn generate_name(i: usize) -> String {
  let mut n = String::new();
  n.push((b'a' + (i % 26) as u8) as char);
  if i >= 26 {
    n += &(i / 26).to_string();
  }
  n
}
