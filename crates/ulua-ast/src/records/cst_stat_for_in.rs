use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatForIn {
  pub base: CstNode,
  pub vars_annotation_colon_positions: AstArray<Position>,
  pub vars_comma_positions: AstArray<Position>,
  pub values_comma_positions: AstArray<Position>,
}

impl_cst_node_class!(CstStatForIn);
impl_cst_node_new!(
  CstStatForIn,
  vars_annotation_colon_positions: AstArray<Position>,
  vars_comma_positions: AstArray<Position>,
  values_comma_positions: AstArray<Position>,
);
