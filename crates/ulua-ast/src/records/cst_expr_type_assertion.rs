use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprTypeAssertion {
  pub base: CstNode,
  pub op_position: Position,
}

impl_cst_node_class!(CstExprTypeAssertion);
impl_cst_node_new!(CstExprTypeAssertion, op_position: Position);
