//! Source: `Analysis/src/AstJsonEncoder.cpp:1055-1070` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_type_function::AstTypeFunction};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_type_function(&mut self, node: *mut AstTypeFunction) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeFunction", |e| {
      e.write("attributes", &n.attributes);
      e.write("generics", &n.generics);
      e.write("genericPacks", &n.generic_packs);
      e.write("argTypes", &n.arg_types);
      e.write("argNames", &n.arg_names);
      e.write("returnTypes", &n.return_types);
    });
  }
}
