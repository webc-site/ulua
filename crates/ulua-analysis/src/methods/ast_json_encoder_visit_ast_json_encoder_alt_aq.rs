//! Source: `Analysis/src/AstJsonEncoder.cpp:1459-1463` (hand-ported)
use ulua_ast::records::ast_type_reference::AstTypeReference;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_reference(&mut self, node: *mut AstTypeReference) -> bool {
    unsafe { self.write_ast_type_reference(node) };
    false
  }
}
