//! Source: `Analysis/src/AstJsonEncoder.cpp:992-1008` (hand-ported)
use ulua_ast::records::ast_type_reference::AstTypeReference;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// `prefix` 是可空指针槽位（`Option` 仅承载缺省语义），判空后原样下传桥接。
  pub fn write_ast_type_reference(&mut self, node: &AstTypeReference) {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstTypeReference", |e| {
      if node.prefix.is_some() {
        e.write("prefix", &node.prefix);
      }
      if let Some(prefix_location) = node.prefix_location {
        e.write("prefixLocation", &prefix_location);
      }
      e.write("name", &node.name);
      e.write("nameLocation", &node.name_location);
      e.write("parameters", &node.parameters);
    });
  }
}
