use crate::{
  methods::json_emitter_write_raw_json_emitter::append_chunk,
  records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  pub fn append_chunk(&mut self, sv: &str) {
    append_chunk(&mut self.chunks, sv);
  }
}
