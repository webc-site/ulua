use alloc::string::String;

use crate::records::reducer::Reducer;

impl Reducer {
  /// `std::string Reducer::escape(const std::string& s)` (`CLI/src/Reduce.cpp:126-140`):
  /// wrap in quotes and escape embedded `"` for shell embedding.
  pub fn escape(&self, s: &str) -> String {
    // 转义只关心单字节 `"`，按字节透传免 UTF-8 解码
    let mut result = String::with_capacity(s.len() + 20);
    result.push('"');

    for &b in s.as_bytes() {
      if b == b'"' {
        result.push('\\');
      }
      result.push(b as char);
    }

    result.push('"');
    result
  }
}
