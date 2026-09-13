//! Source: `Analysis/src/AstJsonEncoder.cpp:891-905` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_type_alias::AstStatTypeAlias};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_type_alias(&mut self, node: *mut AstStatTypeAlias) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatTypeAlias", |e| {
      e.write("name", &n.name);
      e.write("generics", &n.generics);
      e.write("genericPacks", &n.generic_packs);
      e.write("value", &n.type_ptr);
      e.write("exported", &n.exported);
    });
  }
}
