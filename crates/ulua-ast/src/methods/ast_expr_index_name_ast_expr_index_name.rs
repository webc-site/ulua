use core::ffi::c_char;

use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_index_name::AstExprIndexName, ast_name::AstName, ast_node::AstNode,
    location::Location, position::Position,
  },
  rtti::AstNodeClass,
};

impl AstExprIndexName {
  pub fn new(
    location: Location,
    expr: *mut AstExpr,
    index: AstName,
    index_location: Location,
    op_position: Position,
    op: c_char,
  ) -> Self {
    Self {
      base: AstExpr {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      expr,
      index,
      index_location,
      op_position,
      op,
    }
  }
}

pub fn ast_expr_index_name_ast_expr_index_name(
  location: Location,
  expr: *mut AstExpr,
  index: AstName,
  index_location: Location,
  op_position: Position,
  op: c_char,
) -> AstExprIndexName {
  AstExprIndexName::new(location, expr, index, index_location, op_position, op)
}
