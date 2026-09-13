use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError, ast_node::AstNode,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstExprError {
  pub fn new(location: Location, expressions: AstArray<*mut AstExpr>, message_index: u32) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expressions,
      message_index,
    }
  }
}

pub fn ast_expr_error_ast_expr_error(
  location: Location,
  expressions: AstArray<*mut AstExpr>,
  message_index: u32,
) -> AstExprError {
  AstExprError::new(location, expressions, message_index)
}
