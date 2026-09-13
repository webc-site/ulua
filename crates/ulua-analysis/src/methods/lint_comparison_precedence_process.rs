use ulua_ast::visit::ast_stat_visit;

use crate::records::{
  lint_comparison_precedence::LintComparisonPrecedence, lint_context::LintContext,
};
impl LintComparisonPrecedence {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintComparisonPrecedence {
      context: context as *mut LintContext,
    };
    unsafe {
      ast_stat_visit(context.root, &mut pass);
    }
  }
}
