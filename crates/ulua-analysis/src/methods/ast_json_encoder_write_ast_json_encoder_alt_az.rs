//! Source: `Analysis/src/AstJsonEncoder.cpp:714-728` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_if::AstStatIf};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_if(&mut self, node: *mut AstStatIf) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatIf", |e| {
      e.write("condition", &n.condition);
      e.write("thenbody", &n.thenbody);
      if !n.elsebody.is_null() {
        e.write("elsebody", &n.elsebody);
      }
      e.write("hasThen", &n.then_location.is_some());
    });
  }
}
