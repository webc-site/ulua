use core::str::from_utf8_unchecked;

use crate::{
  methods::json_emitter_write_raw_json_emitter::AsRawByte,
  records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  // writeRaw(std::string_view) — pinned overload name
  pub fn write_raw_string_view(&mut self, sv: &str) {
    self.append_chunk(sv);
  }

  // writeRaw(char) — pinned overload name
  pub fn write_raw_c_char<B: AsRawByte>(&mut self, c: B) {
    let buf = [c.to_raw_byte()];
    // single char as a one-byte string view
    self.write_raw_string_view(unsafe { from_utf8_unchecked(&buf) });
  }
}
