use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeTypeof {
  pub base: CstNode,
  pub open_position: Position,
  pub close_position: Position,
}

impl_cst_node_class!(CstTypeTypeof);
impl_cst_node_new!(CstTypeTypeof, open_position: Position, close_position: Position);
