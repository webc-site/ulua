//! Source: `Analysis/src/AstJsonEncoder.cpp:730-742` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_while::AstStatWhile};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_while(&mut self, node: *mut AstStatWhile) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatWhile", |e| {
      e.write("condition", &n.condition);
      e.write("body", &n.body);
      e.write("hasDo", &n.has_do);
    });
  }
}
