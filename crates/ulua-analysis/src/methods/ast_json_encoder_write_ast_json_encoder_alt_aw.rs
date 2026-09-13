//! Source: `Analysis/src/AstJsonEncoder.cpp:663-674` (hand-ported)
use ulua_ast::records::{ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_type_assertion(&mut self, node: *mut AstExprTypeAssertion) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprTypeAssertion", |e| {
      e.write("expr", &n.expr);
      e.write("annotation", &n.annotation);
    });
  }
}
