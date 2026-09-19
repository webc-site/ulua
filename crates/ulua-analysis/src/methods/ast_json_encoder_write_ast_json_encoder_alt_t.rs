//! Source: `Analysis/src/AstJsonEncoder.cpp:271-274` (hand-ported)
use ulua_ast::{records::ast_node::AstNode, visit::ast_node_visit};

use crate::records::ast_json_encoder::AstJsonEncoder;
impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // C++ write(AstNode* node) { node->visit(this); } -- virtual dispatch on
  // the node's dynamic type, landing in this encoder's visit overrides.
  pub unsafe fn write_ast_node(&mut self, node: *mut AstNode) {
    unsafe {
      ast_node_visit(node, self);
    }
  }
}
