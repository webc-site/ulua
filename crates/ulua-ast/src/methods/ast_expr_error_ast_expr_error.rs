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
