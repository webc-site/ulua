use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstTypePackExplicit {
  pub base: CstNode,
  pub open_parentheses_position: Position,
  pub close_parentheses_position: Position,
  pub comma_positions: AstArray<Position>,
}

impl CstNodeClass for CstTypePackExplicit {
  const CLASS_INDEX: i32 = ast_rtti_index("CstTypePackExplicit");
}
