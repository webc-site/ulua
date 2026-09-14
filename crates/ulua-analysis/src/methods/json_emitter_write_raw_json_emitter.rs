use crate::records::json_emitter::JsonEmitter;

impl JsonEmitter {
  pub fn write_raw_string_view(&mut self, sv: &str) {
    const CHUNK_SIZE: usize = 4096;

    if sv.len() > CHUNK_SIZE {
      self.chunks.push(sv.to_string());
      self.new_chunk();
      return;
    }

    let sv_len = sv.len();
    let chunk_len = self.chunks.last().map_or(0, |c| c.len());

    if chunk_len + sv_len < CHUNK_SIZE {
      let chunk = self.chunks.last_mut().unwrap();
      chunk.push_str(sv);
      return;
    }

    let prefix = CHUNK_SIZE - chunk_len;
    let chunk = self.chunks.last_mut().unwrap();
    chunk.push_str(&sv[..prefix]);

    self.new_chunk();

    self.chunks.last_mut().unwrap().push_str(&sv[prefix..]);
  }
}
