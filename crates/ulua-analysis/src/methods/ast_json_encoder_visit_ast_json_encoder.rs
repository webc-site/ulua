//! Source: `Analysis/src/AstJsonEncoder.cpp:1174-1185` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_group::AstTypeGroup};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_type_group(&mut self, node: *mut AstTypeGroup) -> bool {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeGroup", |e| {
      e.write("inner", &n.type_);
    });
    false
  }
}
