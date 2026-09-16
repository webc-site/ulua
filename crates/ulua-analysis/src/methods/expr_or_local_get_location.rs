use ulua_ast::records::location::Location;

use crate::records::expr_or_local::ExprOrLocal;

impl ExprOrLocal {
  pub fn get_location(&self) -> Option<Location> {
    let expr = self.get_expr();
    if !expr.is_null() {
      return Some(unsafe { (*expr).base.location });
    }

    let local = self.get_local();
    if !local.is_null() {
      return Some(unsafe { (*local).location });
    }

    None
  }
}
