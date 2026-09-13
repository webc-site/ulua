//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:83:write_type`
//! Source: `Analysis/src/AstJsonEncoder.cpp` (AstJsonEncoder.cpp:83-86, hand-ported)

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  // writeType(std::string_view propValue) — write("type", propValue) expansion
  pub fn write_type_string_view(&mut self, prop_value: &str) {
    if self.comma {
      self.write_raw_string_view(",");
    }
    self.comma = true;
    self.write_raw_string_view("\"type\":");
    // write(std::string_view) — JSON-escaped string write
    self.write_raw_string_view("\"");
    self.write_raw_string_view(prop_value);
    self.write_raw_string_view("\"");
  }
}
