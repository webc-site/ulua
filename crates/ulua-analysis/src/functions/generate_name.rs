extern crate alloc;

use alloc::string::{String, ToString};

pub fn generate_name(i: usize) -> String {
  let mut n = String::new();
  n.push((b'a' + (i % 26) as u8) as char);
  if i >= 26 {
    n += &(i / 26).to_string();
  }
  n
}
