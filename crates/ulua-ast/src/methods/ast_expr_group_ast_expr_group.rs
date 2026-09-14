use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_group::AstExprGroup, ast_node::AstNode, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprGroup {
  pub fn new(location: Location, expr: *mut AstExpr) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expr,
    }
  }
}

pub fn ast_expr_group_ast_expr_group(location: Location, expr: *mut AstExpr) -> AstExprGroup {
  AstExprGroup::new(location, expr)
}
