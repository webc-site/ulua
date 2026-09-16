use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

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

impl CstNodeClass for CstTypeFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypeFunction");
}
