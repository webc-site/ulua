use ulua_ast::records::ast_type_pack::AstTypePack;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
    let _ = node;
    true
  }
}
