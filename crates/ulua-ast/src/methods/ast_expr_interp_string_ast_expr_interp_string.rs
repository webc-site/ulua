use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_expr_interp_string::AstExprInterpString,
  location::Location,
};

impl_ast_node_new!(
  AstExprInterpString,
  AstExpr,
  location: Location,
  strings: AstArray<AstArray<u8>>,
  expressions: AstArray<*mut AstExpr>,
);
