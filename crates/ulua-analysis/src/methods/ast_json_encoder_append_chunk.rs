use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn append_chunk(&mut self, sv: &str) {
    const CHUNK_SIZE: usize = 4096;

    if sv.len() > CHUNK_SIZE {
      self.chunks.push(sv.to_string());
      self.new_chunk();
      return;
    }

    if self.chunks.is_empty() {
      self.new_chunk();
    }

    let chunk = self.chunks.last_mut().unwrap();
    if chunk.len() + sv.len() < CHUNK_SIZE {
      chunk.push_str(sv);
      return;
    }

    let prefix = CHUNK_SIZE - chunk.len();
    chunk.push_str(&sv[..prefix]);
    self.new_chunk();

    self.chunks.last_mut().unwrap().push_str(&sv[prefix..]);
  }
}
