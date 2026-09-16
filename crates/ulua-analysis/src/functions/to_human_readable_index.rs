extern crate alloc;

use alloc::string::{String, ToString};

pub fn to_human_readable_index(number: usize) -> String {
  let human_index = number + 1;
  let final_digit = human_index % 10;

  if human_index > 10 && human_index < 20 {
    let mut s = human_index.to_string();
    s.push_str("th");
    return s;
  }

  let mut s = human_index.to_string();
  match final_digit {
    1 => s.push_str("st"),
    2 => s.push_str("nd"),
    3 => s.push_str("rd"),
    _ => s.push_str("th"),
  }
  s
}
