use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_unknown_type::LintUnknownType};
pub fn lint_unknown_type_process(context: &mut LintContext) {
  let mut pass = LintUnknownType {
    context: context as *mut LintContext,
  };
  unsafe { ast_stat_visit(context.root, &mut pass) };
}
