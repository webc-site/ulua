use ulua_ast::records::ast_stat_function::AstStatFunction;

use crate::records::lint_deprecated_api::LintDeprecatedApi;

impl LintDeprecatedApi {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    unsafe {
      self.check_ast_expr_function((*node).func);
    }

    false
  }
}
