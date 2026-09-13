use crate::{
  functions::is_constant_literal::is_constant_literal,
  records::{
    ast_expr::AstExpr,
    ast_expr_table::{AstExprTable, ItemKind},
    ast_node::AstNode,
  },
  rtti::ast_node_as,
};

pub fn is_literal_table(expr: *mut AstExpr) -> bool {
  if expr.is_null() {
    return false;
  }

  let table = unsafe { ast_node_as::<AstExprTable>(expr as *mut AstNode) };
  if table.is_null() {
    return false;
  }

  let items = unsafe { &(*table).items };
  for item in items.iter() {
    match item.kind {
      ItemKind::General => {
        return false;
      }
      ItemKind::Record | ItemKind::List => {
        if !is_constant_literal(item.value) && !is_literal_table(item.value) {
          return false;
        }
      }
    }
  }

  true
}
