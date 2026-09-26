use crate::{
  functions::write_json_emitter::write_string, records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  pub fn write_string(&mut self, sv: &str) {
    write_string(sv, |part| self.write_raw_string_view(part));
  }
}
