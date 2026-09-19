use alloc::string::String;

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  pub fn str(&mut self) -> String {
    self.chunks.join("")
  }
}
