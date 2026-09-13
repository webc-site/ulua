use ulua_ast::visit::ast_stat_visit;

use crate::records::lint_duplicate_condition::LintDuplicateCondition;

impl LintDuplicateCondition {
  pub fn process(&mut self) {
    unsafe {
      ast_stat_visit((*self.context).root, self);
    }
  }
}
