use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatDo {
  pub base: CstNode,
  pub stats_start_position: Position,
  pub end_position: Position,
}

impl_cst_node_class!(CstStatDo);
impl_cst_node_new!(CstStatDo, stats_start_position: Position, end_position: Position);
