//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:78:ast_json_encode`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:78-81, hand-ported)

use core::{ffi::c_char, str::from_utf8_unchecked};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  // writeRaw(char) — pinned overload name
  pub fn write_raw_c_char(&mut self, c: c_char) {
    let buf = [c as u8];
    // single char as a one-byte string view
    self.write_raw_string_view(unsafe { from_utf8_unchecked(&buf) });
  }
}
