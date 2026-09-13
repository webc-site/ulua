use ulua_ast::visit::ast_stat_visit;

use crate::records::{
  lint_context::LintContext, lint_unbalanced_assignment::LintUnbalancedAssignment,
};
#[inline(never)]
pub fn lint_unbalanced_assignment_process(context: &mut LintContext) {
  let mut pass = LintUnbalancedAssignment {
    context: context as *mut LintContext,
  };
  unsafe { ast_stat_visit(context.root, &mut pass) };
}
