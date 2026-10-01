use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};

use crate::records::arena_handle::alias_opt;

pub(crate) fn unwrap_group(mut expr: *mut AstExpr) -> *mut AstExpr {
  while let Some(AstExprRef::Group(group)) = alias_opt(expr).map(AstExpr::as_expr_ref) {
    expr = group.expr.as_ptr();
  }

  expr
}
