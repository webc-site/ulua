use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

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

impl_cst_node_class!(CstStatTypeAlias);
impl_cst_node_new!(
  CstStatTypeAlias,
  type_keyword_position: Position,
  generics_open_position: Position,
  generics_comma_positions: AstArray<Position>,
  generics_close_position: Position,
  equals_position: Position,
);
