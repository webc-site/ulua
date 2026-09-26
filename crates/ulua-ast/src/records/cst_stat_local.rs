use crate::{
  records::{ast_array::AstArray, cst_node::CstNode, position::Position},
  rtti::CstNodeClass,
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

impl_cst_node_class!(CstStatLocal);

impl CstStatLocal {
  pub fn new(
    vars_annotation_colon_positions: AstArray<Position>,
    vars_comma_positions: AstArray<Position>,
    values_comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode::new(<Self as CstNodeClass>::CLASS_INDEX),
      declaration_keyword_position: Position::missing(),
      vars_annotation_colon_positions,
      vars_comma_positions,
      values_comma_positions,
    }
  }
}
