use crate::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_expr::AstStatExpr, location::Location,
  node_handle::Node,
};

impl_ast_node_new!(AstStatExpr, AstStat, location: Location, expr: Node<AstExpr>);
