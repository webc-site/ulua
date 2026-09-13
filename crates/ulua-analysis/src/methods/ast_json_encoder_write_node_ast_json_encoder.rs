use ulua_ast::records::ast_node::AstNode;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn write_node_ast_node(&mut self, node: *mut AstNode) {
    unsafe {
      self.write("location", &(*node).location);
    }
  }
}
