use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypeFunction {
  pub base: CstNode,
  pub open_generics_position: Position,
  pub generics_comma_positions: AstArray<Position>,
  pub close_generics_position: Position,
  pub open_args_position: Position,
  pub argument_name_colon_positions: AstArray<Position>,
  pub arguments_comma_positions: AstArray<Position>,
  pub close_args_position: Position,
  pub return_arrow_position: Position,
}

impl_cst_node_class!(CstTypeFunction);
impl_cst_node_new!(
  CstTypeFunction,
  open_generics_position: Position,
  generics_comma_positions: AstArray<Position>,
  close_generics_position: Position,
  open_args_position: Position,
  argument_name_colon_positions: AstArray<Position>,
  arguments_comma_positions: AstArray<Position>,
  close_args_position: Position,
  return_arrow_position: Position,
);
