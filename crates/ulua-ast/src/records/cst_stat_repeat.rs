use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatRepeat {
  pub base: CstNode,
  pub until_position: Position,
}

impl_cst_node_class!(CstStatRepeat);
impl_cst_node_new!(CstStatRepeat, until_position: Position);
