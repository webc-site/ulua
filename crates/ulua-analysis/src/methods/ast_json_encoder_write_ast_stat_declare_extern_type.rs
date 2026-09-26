//! Source: `Analysis/src/AstJsonEncoder.cpp:955-969` (hand-ported)
use ulua_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  pub fn write_ast_stat_declare_extern_type(&mut self, node: &AstStatDeclareExternType) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstStatDeclareClass", |e| {
      e.write("name", &node.name);
      if let Some(super_name) = node.super_name {
        e.write("superName", &super_name);
      }
      e.write("props", &node.props);
      e.write("indexer", &node.indexer);
    });
  }
}
