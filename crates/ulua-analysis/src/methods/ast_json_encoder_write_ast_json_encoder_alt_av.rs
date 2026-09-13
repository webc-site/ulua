//! Source: `Analysis/src/AstJsonEncoder.cpp:649-661` (hand-ported)
use ulua_ast::records::{ast_expr_binary::AstExprBinary, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_binary(&mut self, node: *mut AstExprBinary) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprBinary", |e| {
      e.write("op", &n.op);
      e.write("left", &n.left);
      e.write("right", &n.right);
    });
  }
}
