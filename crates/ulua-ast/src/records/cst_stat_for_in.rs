#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatForIn {
  pub base: CstNode,
  pub vars_annotation_colon_positions: AstArray<Position>,
  pub vars_comma_positions: AstArray<Position>,
  pub values_comma_positions: AstArray<Position>,
}

impl CstNodeClass for CstStatForIn {
  const CLASS_INDEX: i32 = cst_rtti_index("CstStatForIn");
}
use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, cst_rtti_index},
};
