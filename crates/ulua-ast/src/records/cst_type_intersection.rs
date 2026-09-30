use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeIntersection {
  pub base: CstNode,
  pub leading_position: Position,
  pub separator_positions: AstArray<Position>,
}

impl_cst_node_class!(CstTypeIntersection);
impl_cst_node_new!(
  CstTypeIntersection,
  leading_position: Position,
  separator_positions: AstArray<Position>,
);
