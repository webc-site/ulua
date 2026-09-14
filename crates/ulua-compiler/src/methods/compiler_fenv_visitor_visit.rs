use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::fenv_visitor::FenvVisitor;

impl FenvVisitor<'_> {
  pub fn visit(&mut self, node: &AstExprGlobal) -> bool {
    let name_bytes = node.name.as_bytes();
    if name_bytes == b"getfenv" {
      *self.getfenv_used = true;
    }
    if name_bytes == b"setfenv" {
      *self.setfenv_used = true;
    }
    false
  }
}
