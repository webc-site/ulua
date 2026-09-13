//! Source: `Analysis/src/AstJsonEncoder.cpp:1200-1211` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_singleton_string::AstTypeSingletonString};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type_singleton_string(
    &mut self,
    node: *mut AstTypeSingletonString,
  ) -> bool {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeSingletonString", |e| {
      e.write("value", &n.value);
    });
    false
  }
}
