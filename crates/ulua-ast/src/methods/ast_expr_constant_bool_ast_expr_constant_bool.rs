use crate::records::{
  ast_expr::AstExpr, ast_expr_constant_bool::AstExprConstantBool, location::Location,
};

impl_ast_node_new!(AstExprConstantBool, AstExpr, location: Location, value: bool);
