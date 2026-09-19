//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:73-76, hand-ported)

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  // writeRaw(std::string_view) — pinned overload name
  pub fn write_raw_string_view(&mut self, sv: &str) {
    self.append_chunk(sv);
  }
}
