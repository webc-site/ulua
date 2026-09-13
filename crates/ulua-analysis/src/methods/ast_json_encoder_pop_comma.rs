use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn pop_comma(&mut self, c: bool) {
    self.comma = c;
  }
}
