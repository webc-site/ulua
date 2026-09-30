use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprGroup {
  pub base: CstNode,
  pub close_position: Position,
}

impl_cst_node_class!(CstExprGroup);
impl_cst_node_new!(CstExprGroup, close_position: Position);
