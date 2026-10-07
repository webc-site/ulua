use crate::records::{ast_array::AstArray, cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstExprInterpString {
  pub base: CstNode,
  pub source_strings: AstArray<AstArray<u8>>,
  pub string_positions: AstArray<Position>,
}

impl_cst_node_class!(CstExprInterpString);
impl_cst_node_new!(
  CstExprInterpString,
  source_strings: AstArray<AstArray<u8>>,
  string_positions: AstArray<Position>,
);
