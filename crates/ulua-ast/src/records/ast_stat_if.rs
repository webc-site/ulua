use crate::records::{
  ast_expr::AstExpr,
  ast_stat::AstStat,
  ast_stat_block::AstStatBlock,
  location::Location,
  node_handle::{Node, OptNode},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatIf {
  pub base: AstStat,
  pub condition: Node<AstExpr>,
  pub thenbody: Node<AstStatBlock>,
  pub elsebody: OptNode<AstStat>,
  pub then_location: Option<Location>,
  pub else_location: Option<Location>,
}
