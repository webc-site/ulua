use crate::records::{
  ast_expr::AstExpr, ast_type::AstType, ast_type_typeof::AstTypeTypeof, location::Location,
};

impl_ast_node_new!(AstTypeTypeof, AstType, location: Location, expr: *mut AstExpr);
