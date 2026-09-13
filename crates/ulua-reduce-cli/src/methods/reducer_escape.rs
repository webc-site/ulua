use alloc::string::String;

use crate::records::reducer::Reducer;

pub fn reducer_escape(_this: &Reducer, s: &str) -> String {
  let mut result = String::with_capacity(s.len() + 20);
  result.push('"');

  for c in s.chars() {
    if c == '"' {
      result.push('\\');
    }
    result.push(c);
  }

  result.push('"');
  result
}
