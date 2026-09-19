use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_varargs::AstExprVarargs, ast_node::AstNode, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprVarargs {
  pub fn new(location: Location) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
    }
  }
}
