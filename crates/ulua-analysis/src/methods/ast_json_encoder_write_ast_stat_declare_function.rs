//! Source: `Analysis/src/AstJsonEncoder.cpp:907-926` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_declare_function::AstStatDeclareFunction};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_declare_function(&mut self, node: *mut AstStatDeclareFunction) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatDeclareFunction", |e| {
      e.write("attributes", &n.attributes);
      e.write("name", &n.name);
      e.write("nameLocation", &n.name_location);
      e.write("params", &n.params);
      e.write("paramNames", &n.param_names);
      e.write("vararg", &n.vararg);
      e.write("vararg_location", &n.vararg_location);
      e.write("retTypes", &n.ret_types);
      e.write("generics", &n.generics);
      e.write("genericPacks", &n.generic_packs);
    });
  }
}
