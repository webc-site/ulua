//! Source: `Analysis/src/AstJsonEncoder.cpp:541-555` (hand-ported)
use ulua_ast::records::{ast_expr_if_else::AstExprIfElse, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_if_else(&mut self, node: *mut AstExprIfElse) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprIfElse", |e| {
      e.write("condition", &n.condition);
      e.write("hasThen", &n.has_then);
      e.write("trueExpr", &n.true_expr);
      e.write("hasElse", &n.has_else);
      e.write("falseExpr", &n.false_expr);
    });
  }
}
