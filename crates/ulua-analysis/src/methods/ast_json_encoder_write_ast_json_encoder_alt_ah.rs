//! Source: `Analysis/src/AstJsonEncoder.cpp:424-435` (hand-ported)
use ulua_ast::records::{ast_expr_index_expr::AstExprIndexExpr, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_index_expr(&mut self, node: *mut AstExprIndexExpr) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprIndexExpr", |e| {
      e.write("expr", &n.expr);
      e.write("index", &n.index);
    });
  }
}
