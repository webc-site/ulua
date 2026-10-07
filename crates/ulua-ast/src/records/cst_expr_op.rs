use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprOp {
  pub base: CstNode,
  pub op_position: Position,
}

impl_cst_node_class!(CstExprOp);
impl_cst_node_new!(CstExprOp, op_position: Position);
