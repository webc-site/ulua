use ulua_ast::records::ast_type_pack_explicit::AstTypePackExplicit;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_pack_explicit(&mut self, node: *mut AstTypePackExplicit) -> bool {
    unsafe { self.write_ast_type_pack_explicit(node) };
    false
  }
}
