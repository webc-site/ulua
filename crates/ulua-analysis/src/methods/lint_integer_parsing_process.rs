use ulua_ast::visit::ast_stat_visit;

use crate::records::{lint_context::LintContext, lint_integer_parsing::LintIntegerParsing};
impl LintIntegerParsing {
  pub fn process(context: &mut LintContext) {
    let mut pass = LintIntegerParsing {
      context: context as *mut LintContext,
    };
    unsafe {
      ast_stat_visit(context.root, &mut pass);
    }
  }
}
