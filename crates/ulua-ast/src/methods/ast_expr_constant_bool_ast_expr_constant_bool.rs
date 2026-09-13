use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprConstantBool {
  pub fn new(location: Location, value: bool) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
      },
      value,
    }
  }
}

pub fn ast_expr_constant_bool_ast_expr_constant_bool(
  location: Location,
  value: bool,
) -> AstExprConstantBool {
  AstExprConstantBool::new(location, value)
}
