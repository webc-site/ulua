//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:508:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp:508-519` (hand-ported)
use ulua_ast::records::ast_expr_table::ItemKind;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_expr_table_item_kind(&mut self, kind: ItemKind) {
    match kind {
      ItemKind::List => self.write_string("item"),
      ItemKind::Record => self.write_string("record"),
      ItemKind::General => self.write_string("general"),
    }
  }
}
