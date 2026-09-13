use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_function::CstTypeFunction, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstTypeFunction {
  pub fn new(
    open_generics_position: Position,
    generics_comma_positions: AstArray<Position>,
    close_generics_position: Position,
    open_args_position: Position,
    argument_name_colon_positions: AstArray<Position>,
    arguments_comma_positions: AstArray<Position>,
    close_args_position: Position,
    return_arrow_position: Position,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      open_generics_position,
      generics_comma_positions,
      close_generics_position,
      open_args_position,
      argument_name_colon_positions,
      arguments_comma_positions,
      close_args_position,
      return_arrow_position,
    }
  }
}

pub fn cst_type_function_cst_type_function(
  open_generics_position: Position,
  generics_comma_positions: AstArray<Position>,
  close_generics_position: Position,
  open_args_position: Position,
  argument_name_colon_positions: AstArray<Position>,
  arguments_comma_positions: AstArray<Position>,
  close_args_position: Position,
  return_arrow_position: Position,
) -> CstTypeFunction {
  CstTypeFunction::new(
    open_generics_position,
    generics_comma_positions,
    close_generics_position,
    open_args_position,
    argument_name_colon_positions,
    arguments_comma_positions,
    close_args_position,
    return_arrow_position,
  )
}
