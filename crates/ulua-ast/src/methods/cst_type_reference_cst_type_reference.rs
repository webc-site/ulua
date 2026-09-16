use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_reference::CstTypeReference,
    position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypeReference {
  pub fn new(
    prefix_point_position: Position,
    open_parameters_position: Position,
    parameters_comma_positions: AstArray<Position>,
    close_parameters_position: Position,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      prefix_point_position,
      open_parameters_position,
      parameters_comma_positions,
      close_parameters_position,
    }
  }
}

pub fn cst_type_reference_cst_type_reference(
  prefix_point_position: Position,
  open_parameters_position: Position,
  parameters_comma_positions: AstArray<Position>,
  close_parameters_position: Position,
) -> CstTypeReference {
  CstTypeReference::new(
    prefix_point_position,
    open_parameters_position,
    parameters_comma_positions,
    close_parameters_position,
  )
}
