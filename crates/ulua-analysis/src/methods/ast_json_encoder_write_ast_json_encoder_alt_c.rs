//! Source: `Analysis/src/AstJsonEncoder.cpp:108-131` (hand-ported)
use ulua_common::functions::format_g::format_g;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_f64(&mut self, d: f64) {
    if d.is_infinite() {
      self.write_raw_string_view(if d < 0.0 { "-Infinity" } else { "Infinity" });
    } else if d.is_nan() {
      self.write_raw_string_view("NaN");
    } else {
      let formatted = format_g(d, 17);
      self.write_raw_string_view(&formatted);
    }
  }
}
