use ulua_ast::records::ast_type::AstType;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  pub fn visit_ast_type(&mut self, node: *mut AstType) -> bool {
    let _ = node;
    true
  }
}
