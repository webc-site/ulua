#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeReference {
  pub base: CstNode,
  pub prefix_point_position: Position,
  pub open_parameters_position: Position,
  pub parameters_comma_positions: AstArray<Position>,
  pub close_parameters_position: Position,
}

impl CstNodeClass for CstTypeReference {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeReference");
}
use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
