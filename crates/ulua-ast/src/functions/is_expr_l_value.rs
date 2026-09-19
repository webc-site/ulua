use crate::{
  functions::is_l_value::l_value_kind,
  records::{ast_expr::AstExpr, ast_node::AstNode},
};

pub fn is_expr_l_value(expr: *mut AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  unsafe {
    l_value_kind(expr as *mut AstNode as *const AstNode, |local_expr| {
      !(*local_expr).local.is_null() && !(*(*local_expr).local).is_const
    })
  }
}
