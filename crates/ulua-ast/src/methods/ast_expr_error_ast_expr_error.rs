use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError, location::Location,
};

impl_ast_node_new!(
  AstExprError,
  AstExpr,
  location: Location,
  expressions: AstArray<*mut AstExpr>,
  message_index: u32,
);
