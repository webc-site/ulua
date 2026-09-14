//! Source: `Analysis/src/AstJsonEncoder.cpp:865-876` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_function::AstStatFunction};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_function(&mut self, node: *mut AstStatFunction) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatFunction", |e| {
      e.write("name", &n.name);
      e.write("func", &n.func);
    });
  }
}
