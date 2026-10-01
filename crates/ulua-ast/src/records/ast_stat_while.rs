use crate::records::{
  ast_expr::AstExpr, ast_stat::AstStat, ast_stat_block::AstStatBlock, location::Location,
  node_handle::Node,
};

#[repr(C)]
#[derive(Debug)]
pub struct AstStatWhile {
  pub base: AstStat,
  pub condition: Node<AstExpr>,
  pub body: Node<AstStatBlock>,
  pub has_do: bool,
  pub do_location: Location,
}
