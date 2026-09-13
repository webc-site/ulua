//! Source: `Analysis/src/AstJsonEncoder.cpp:992-1008` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_reference::AstTypeReference};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_type_reference(&mut self, node: *mut AstTypeReference) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeReference", |e| {
      if n.prefix.is_some() {
        e.write("prefix", &n.prefix);
      }
      if let Some(prefix_location) = n.prefix_location {
        e.write("prefixLocation", &prefix_location);
      }
      e.write("name", &n.name);
      e.write("nameLocation", &n.name_location);
      e.write("parameters", &n.parameters);
    });
  }
}
