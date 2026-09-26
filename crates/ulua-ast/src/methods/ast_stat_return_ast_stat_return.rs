use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat, ast_stat_return::AstStatReturn,
  location::Location,
};

impl_ast_node_new!(AstStatReturn, AstStat, location: Location, list: AstArray<*mut AstExpr>);
