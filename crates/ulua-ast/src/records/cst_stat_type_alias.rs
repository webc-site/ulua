#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatTypeAlias {
  pub base: CstNode,
  pub type_keyword_position: Position,
  pub generics_open_position: Position,
  pub generics_comma_positions: AstArray<Position>,
  pub generics_close_position: Position,
  pub equals_position: Position,
}

impl CstNodeClass for CstStatTypeAlias {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatTypeAlias");
}
use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};
