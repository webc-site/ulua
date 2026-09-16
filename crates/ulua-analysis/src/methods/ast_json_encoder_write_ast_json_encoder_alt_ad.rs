//! Source: `Analysis/src/AstJsonEncoder.cpp:370-385` (hand-ported)
use ulua_ast::records::ast_array::AstArray;

use crate::{
  methods::ast_json_encoder_write_primitives::WriteJson, records::ast_json_encoder::AstJsonEncoder,
};

// C++ template write(AstArray<T>): "[" elem ("," elem)* "]". The AstArray<char>
// specialization lives in `write_ast_array_c_char` (its own bridge impl), which
// is sound because c_char never implements WriteJson directly.
impl<T: WriteJson> WriteJson for AstArray<T> {
  fn write_json(&self, enc: &mut AstJsonEncoder) {
    enc.write_raw_string_view("[");
    let mut comma = false;
    for a in self.iter() {
      if comma {
        enc.write_raw_string_view(",");
      } else {
        comma = true;
      }
      a.write_json(enc);
    }
    enc.write_raw_string_view("]");
  }
}
