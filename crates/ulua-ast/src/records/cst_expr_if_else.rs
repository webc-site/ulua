use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprIfElse {
  pub base: CstNode,
  pub then_position: Position,
  pub else_position: Position,
  pub is_else_if: bool,
}

impl_cst_node_class!(CstExprIfElse);
impl_cst_node_new!(
  CstExprIfElse,
  then_position: Position,
  else_position: Position,
  is_else_if: bool,
);
