use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeReference {
  pub base: CstNode,
  pub prefix_point_position: Position,
  pub open_parameters_position: Position,
  pub parameters_comma_positions: AstArray<Position>,
  pub close_parameters_position: Position,
}

impl_cst_node_class!(CstTypeReference);
impl_cst_node_new!(
  CstTypeReference,
  prefix_point_position: Position,
  open_parameters_position: Position,
  parameters_comma_positions: AstArray<Position>,
  close_parameters_position: Position,
);
