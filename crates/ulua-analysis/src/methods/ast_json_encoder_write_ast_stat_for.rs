//! Source: `Analysis/src/AstJsonEncoder.cpp:804-820` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_for::AstStatFor};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_for(&mut self, node: *mut AstStatFor) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatFor", |e| {
      e.write("var", &n.var);
      e.write("from", &n.from);
      e.write("to", &n.to);
      if !n.step.is_null() {
        e.write("step", &n.step);
      }
      e.write("body", &n.body);
      e.write("hasDo", &n.has_do);
    });
  }
}
