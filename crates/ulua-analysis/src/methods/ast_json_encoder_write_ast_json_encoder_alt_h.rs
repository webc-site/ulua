use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_i32(&mut self, i: u32) {
    let s = i.to_string();
    self.write_raw_string_view(&s);
  }
}
