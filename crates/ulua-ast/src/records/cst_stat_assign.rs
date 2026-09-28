use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatAssign {
  pub base: CstNode,
  pub vars_comma_positions: AstArray<Position>,
  pub equals_position: Position,
  pub values_comma_positions: AstArray<Position>,
}

impl_cst_node_class!(CstStatAssign);
impl_cst_node_new!(
  CstStatAssign,
  vars_comma_positions: AstArray<Position>,
  equals_position: Position,
  values_comma_positions: AstArray<Position>,
);
