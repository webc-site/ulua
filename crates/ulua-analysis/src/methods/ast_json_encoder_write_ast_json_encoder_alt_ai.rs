//! Source: `Analysis/src/AstJsonEncoder.cpp:437-462` (hand-ported)
use ulua_ast::records::{ast_expr_function::AstExprFunction, ast_node::AstNode};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_expr_function(&mut self, node: *mut AstExprFunction) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprFunction", |e| {
      e.write("attributes", &n.attributes);
      e.write("generics", &n.generics);
      e.write("genericPacks", &n.generic_packs);
      if !n.self_.is_null() {
        e.write("self", &n.self_);
      }
      e.write("args", &n.args);
      if !n.return_annotation.is_null() {
        e.write("returnAnnotation", &n.return_annotation);
      }
      e.write("vararg", &n.vararg);
      e.write("vararg_location", &n.vararg_location);
      if !n.vararg_annotation.is_null() {
        e.write("varargAnnotation", &n.vararg_annotation);
      }
      e.write("body", &n.body);
      e.write("functionDepth", &n.function_depth);
      e.write("debugname", &n.debugname);
    });
  }
}
