use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_deprecated_api::LintDeprecatedApi};

impl LintDeprecatedApi {
  #[inline(never)]
  pub fn process(&mut self, context: &mut LintContext) {
    self.lint_deprecated_api(context as *mut LintContext);
    unsafe {
      ast_stat_visit(context.root, self);
    }
  }
}
