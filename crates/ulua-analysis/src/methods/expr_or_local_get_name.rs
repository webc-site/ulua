use ulua_ast::{functions::get_identifier::get_identifier, records::ast_name::AstName};

use crate::records::expr_or_local::ExprOrLocal;

impl ExprOrLocal {
  pub fn get_name(&self) -> Option<AstName> {
    let expr = self.get_expr();
    if !expr.is_null() {
      let name = get_identifier(expr);
      if !name.value.is_null() {
        return Some(name);
      }
    } else {
      let local = self.get_local();
      if !local.is_null() {
        return Some(unsafe { (*local).name });
      }
    }
    None
  }
}
