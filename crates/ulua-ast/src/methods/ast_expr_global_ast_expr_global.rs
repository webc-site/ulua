use crate::records::{
  ast_expr::AstExpr, ast_expr_global::AstExprGlobal, ast_name::AstName, location::Location,
};

impl_ast_node_new!(AstExprGlobal, AstExpr, location: Location, name: AstName);
