use alloc::string::String;

use crate::records::reducer::Reducer;

impl Reducer {
  /// `std::string Reducer::escape(const std::string& s)` (`CLI/src/Reduce.cpp:126-140`):
  /// wrap in quotes and escape embedded `"` for shell embedding.
  pub fn escape(&self, s: &str) -> String {
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
}
