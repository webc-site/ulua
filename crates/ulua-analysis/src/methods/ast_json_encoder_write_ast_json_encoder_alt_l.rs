//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:188:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:188-191, hand-ported)

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  // write(std::string_view str) — writeString(str) expansion
  pub fn write_string_view(&mut self, str: &str) {
    self.write_string(str);
  }
}
