use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_type_instantiation::CstTypeInstantiation,
    position::Position,
  },
  rtti::{CstNodeClass, cst_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprCall {
  pub base: CstNode,
  pub open_parens: Position,
  pub close_parens: Position,
  pub comma_positions: AstArray<Position>,
  pub explicit_types: *mut CstTypeInstantiation,
}

impl CstNodeClass for CstExprCall {
  const CLASS_INDEX: i32 = cst_rtti_index("CstExprCall");
}
