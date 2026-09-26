use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeGroup {
  pub base: CstNode,
  pub close_position: Position,
}

impl_cst_node_class!(CstTypeGroup);
impl_cst_node_new!(CstTypeGroup, close_position: Position);
