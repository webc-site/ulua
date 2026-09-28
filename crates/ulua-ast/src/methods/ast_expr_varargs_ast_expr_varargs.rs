use crate::records::{ast_expr::AstExpr, ast_expr_varargs::AstExprVarargs, location::Location};

impl_ast_node_new!(AstExprVarargs, AstExpr, location: Location);
