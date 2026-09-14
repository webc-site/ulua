use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_implicit_return::LintImplicitReturn};
pub fn lint_implicit_return_process(context: &mut LintContext) {
  let mut pass = LintImplicitReturn {
    context: context as *mut LintContext,
  };
  unsafe { ast_stat_visit(context.root, &mut pass) };
}
