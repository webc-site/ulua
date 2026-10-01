use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatCompoundAssign {
  pub base: CstNode,
  pub op_position: Position,
}

impl_cst_node_class!(CstStatCompoundAssign);
impl_cst_node_new!(CstStatCompoundAssign, op_position: Position);
