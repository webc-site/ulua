//! Source: `Analysis/src/AstJsonEncoder.cpp:928-940` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_declare_global::AstStatDeclareGlobal};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_declare_global(&mut self, node: *mut AstStatDeclareGlobal) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareGlobal", |e| {
      e.write("name", &n.name);
      e.write("nameLocation", &n.name_location);
      e.write("type", &n.type_);
    });
  }
}
