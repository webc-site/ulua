use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn push_comma(&mut self) -> bool {
    let c = self.comma;
    self.comma = false;
    c
  }
}
