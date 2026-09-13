//! Source: `Analysis/src/AstJsonEncoder.cpp:955-969` (hand-ported)
use ulua_ast::records::{
  ast_node::AstNode, ast_stat_declare_extern_type::AstStatDeclareExternType,
};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_declare_extern_type(&mut self, node: *mut AstStatDeclareExternType) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareClass", |e| {
      e.write("name", &n.name);
      if let Some(super_name) = n.super_name {
        e.write("superName", &super_name);
      }
      e.write("props", &n.props);
      e.write("indexer", &n.indexer);
    });
  }
}
