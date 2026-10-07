use crate::records::{
  ast_expr::AstExpr, ast_expr_constant_nil::AstExprConstantNil, location::Location,
};

impl_ast_node_new!(AstExprConstantNil, AstExpr, location: Location);
