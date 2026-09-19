//! Source: `Analysis/src/AstJsonEncoder.cpp:744-755` (hand-ported)
use ulua_ast::records::{ast_node::AstNode, ast_stat_repeat::AstStatRepeat};

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) {
    let n = unsafe { &*node };
    self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatRepeat", |e| {
      e.write("condition", &n.condition);
      e.write("body", &n.body);
    });
  }
}
