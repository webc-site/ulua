use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_for_range::LintForRange};
impl LintForRange {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintForRange {
      context: context as *mut LintContext,
    };
    unsafe {
      ast_stat_visit(context.root, &mut pass);
    }
  }
}
