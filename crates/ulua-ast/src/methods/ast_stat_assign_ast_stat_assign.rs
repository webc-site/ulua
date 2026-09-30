use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat, ast_stat_assign::AstStatAssign,
  location::Location,
};

impl_ast_node_new!(
  AstStatAssign,
  AstStat,
  location: Location,
  vars: AstArray<*mut AstExpr>,
  values: AstArray<*mut AstExpr>,
);
