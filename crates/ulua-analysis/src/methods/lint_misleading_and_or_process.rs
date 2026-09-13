use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_misleading_and_or::LintMisleadingAndOr};
impl LintMisleadingAndOr {
  pub fn process(&mut self, context: &mut LintContext) {
    self.context = context as *mut LintContext;
    unsafe {
      ast_stat_visit(context.root, self);
    }
  }
}
