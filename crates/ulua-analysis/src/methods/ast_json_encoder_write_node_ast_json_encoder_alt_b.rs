//! Source: `Analysis/src/AstJsonEncoder.cpp:259-269` (hand-ported)
use ulua_ast::records::ast_node::AstNode;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  // C++ template writeNode(AstNode*, string_view, F&& f). The [&] lambda
  // becomes FnOnce(&mut Self) so the body can keep writing through the
  // encoder while the wrapper holds the node frame open.
  pub(crate) fn write_node_ast_node_string_view_f<F: FnOnce(&mut Self)>(
    &mut self,
    node: *mut AstNode,
    name: &str,
    f: F,
  ) {
    self.write_raw_string_view("{");
    let c = self.push_comma();
    self.write_type_string_view(name);
    unsafe { self.write_node_ast_node(node) };
    f(self);
    self.pop_comma(c);
    self.write_raw_string_view("}");
  }
}
