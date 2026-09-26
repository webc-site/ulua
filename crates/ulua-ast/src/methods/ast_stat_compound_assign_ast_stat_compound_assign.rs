use crate::records::{
  ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_stat::AstStat,
  ast_stat_compound_assign::AstStatCompoundAssign, location::Location, node_handle::Node,
};

impl_ast_node_new!(
  AstStatCompoundAssign,
  AstStat,
  location: Location,
  op: AstExprBinaryOp,
  var: Node<AstExpr>,
  value: Node<AstExpr>,
);
