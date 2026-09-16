use ulua_ast::records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal};

use crate::records::lint_deprecated_api::LintDeprecatedApi;

impl LintDeprecatedApi {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    unsafe {
      let fty = self.get_function_type(node as *mut AstExpr);
      let should_report = !fty.is_null() && (*fty).is_deprecated_function && !self.in_scope(fty);

      if should_report {
        let name = (*node).name.as_str().unwrap_or("");
        if let Some(info) = (*fty).deprecated_info.as_deref() {
          self.report_function_info(&(*node).base.base.location, name, info);
        } else {
          self.report_function(&(*node).base.base.location, name);
        }
      }
    }

    true
  }
}
