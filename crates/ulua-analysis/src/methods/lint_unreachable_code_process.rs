use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_unreachable_code::LintUnreachableCode};
impl LintUnreachableCode {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintUnreachableCode {
      context: context as *mut LintContext,
    };
    pass.analyze(context.root);
    unsafe {
      ast_stat_visit(context.root, &mut pass);
    }
  }
}
