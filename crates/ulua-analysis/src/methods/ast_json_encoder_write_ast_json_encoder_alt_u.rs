//! Source: `Analysis/src/AstJsonEncoder.cpp:276-286` (hand-ported)
use ulua_ast::records::{ast_expr_group::AstExprGroup, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_group(&mut self, node: *mut AstExprGroup) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprGroup", |e| {
      e.write("expr", &n.expr);
    });
  }
}
