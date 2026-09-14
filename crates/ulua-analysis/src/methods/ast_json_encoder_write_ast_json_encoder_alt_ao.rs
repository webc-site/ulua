//! Source: `Analysis/src/AstJsonEncoder.cpp:521-539` (hand-ported)
use ulua_ast::records::ast_expr_table::{Item, ItemKind};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_expr_table_item(&mut self, item: &Item) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view("AstExprTableItem");
    self.write("kind", &item.kind);
    match item.kind {
      ItemKind::List => {
        self.write("value", &item.value);
      }
      _ => {
        self.write("key", &item.key);
        self.write("value", &item.value);
      }
    }
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
