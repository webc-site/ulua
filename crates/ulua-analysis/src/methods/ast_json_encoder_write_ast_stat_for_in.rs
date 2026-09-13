//! Source: `Analysis/src/AstJsonEncoder.cpp:822-836` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_for_in::AstStatForIn};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_for_in(&mut self, node: *mut AstStatForIn) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatForIn", |e| {
      e.write("vars", &n.vars);
      e.write("values", &n.values);
      e.write("body", &n.body);
      e.write("hasIn", &n.has_in);
      e.write("hasDo", &n.has_do);
    });
  }
}
