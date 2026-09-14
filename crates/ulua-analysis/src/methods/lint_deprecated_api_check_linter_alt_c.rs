use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_function::AstExprFunction},
  visit::ast_expr_visit,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::lint_deprecated_api::LintDeprecatedApi;
impl LintDeprecatedApi {
  pub fn check_ast_expr_function(&mut self, func: *mut AstExprFunction) {
    LUAU_ASSERT!(!func.is_null());

    let fty = self.get_function_type(func as *mut AstExpr);
    let is_deprecated = !fty.is_null() && unsafe { (*fty).is_deprecated_function };

    if is_deprecated {
      self.push_scope(fty);
    }

    unsafe {
      ast_expr_visit(func as *mut AstExpr, self);
    }

    if is_deprecated {
      self.pop_scope(fty);
    }
  }
}
