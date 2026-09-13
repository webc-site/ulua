//! Source: `Analysis/src/AstJsonEncoder.cpp:984-990` (hand-ported)
use ulua_ast::records::ast_type_or_pack::AstTypeOrPack;

use crate::{
  methods::ast_json_encoder_write_primitives::WriteJson, records::ast_json_encoder::AstJsonEncoder,
};

impl AstJsonEncoder {
  pub fn write_ast_type_or_pack(&mut self, node: &AstTypeOrPack) {
    if !node.r#type.is_null() {
      node.r#type.write_json(self);
    } else {
      node.type_pack.write_json(self);
    }
  }
}
