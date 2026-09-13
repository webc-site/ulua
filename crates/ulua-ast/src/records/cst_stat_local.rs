use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::{CstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatLocal {
  pub base: CstNode,
  pub declaration_keyword_position: Position,
  pub vars_annotation_colon_positions: AstArray<Position>,
  pub vars_comma_positions: AstArray<Position>,
  pub values_comma_positions: AstArray<Position>,
}

impl CstNodeClass for CstStatLocal {
  const CLASS_INDEX: i32 = ast_rtti_index("CstStatLocal");
}
