use ulua_ast::records::ast_expr_interp_string::AstExprInterpString;

use crate::records::ast_json_encoder::AstJsonEncoder;

impl AstJsonEncoder {
  /// # Safety
  /// 调用方须保证 `_node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_interp_string(&mut self, _node: *mut AstExprInterpString) -> bool {
    unsafe { self.write_ast_expr_interp_string(_node) };
    false
  }
}
