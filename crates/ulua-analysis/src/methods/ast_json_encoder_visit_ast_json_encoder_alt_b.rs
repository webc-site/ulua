//! Source: `Analysis/src/AstJsonEncoder.cpp:1187-1198` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_singleton_bool::AstTypeSingletonBool};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type_singleton_bool(&mut self, node: *mut AstTypeSingletonBool) -> bool {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeSingletonBool", |e| {
      e.write("value", &n.value);
    });
    false
  }
}
