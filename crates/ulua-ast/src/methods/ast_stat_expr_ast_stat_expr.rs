use crate::{
  records::{
    ast_expr::AstExpr, ast_node::AstNode, ast_stat::AstStat, ast_stat_expr::AstStatExpr,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatExpr {
  pub fn new(location: Location, expr: *mut AstExpr) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      expr,
    }
  }
}

pub fn ast_stat_expr_ast_stat_expr(location: Location, expr: *mut AstExpr) -> AstStatExpr {
  AstStatExpr::new(location, expr)
}
