use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn new_chunk(&mut self) {
    self.chunks.push(String::new());
    if let Some(last_chunk) = self.chunks.last_mut() {
      last_chunk.reserve(1024);
    }
  }
}
