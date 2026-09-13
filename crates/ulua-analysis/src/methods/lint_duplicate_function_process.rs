use ulua_ast::visit::ast_stat_visit;

use crate::records::lint_duplicate_function::LintDuplicateFunction;

impl LintDuplicateFunction {
  pub fn process(&mut self) {
    unsafe {
      ast_stat_visit((*self.context).root, self);
    }
  }
}
