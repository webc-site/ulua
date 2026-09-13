use crate::{
  records::{
    ast_array::AstArray, cst_node::CstNode, cst_stat_local::CstStatLocal, position::Position,
  },
  rtti::CstNodeClass,
};

impl CstStatLocal {
  pub fn new(
    vars_annotation_colon_positions: AstArray<Position>,
    vars_comma_positions: AstArray<Position>,
    values_comma_positions: AstArray<Position>,
  ) -> Self {
    Self {
      base: CstNode {
        class_index: <Self as CstNodeClass>::CLASS_INDEX,
      },
      declaration_keyword_position: Position::missing(),
      vars_annotation_colon_positions,
      vars_comma_positions,
      values_comma_positions,
    }
  }
}

pub fn cst_stat_local_cst_stat_local(
  vars_annotation_colon_positions: AstArray<Position>,
  vars_comma_positions: AstArray<Position>,
  values_comma_positions: AstArray<Position>,
) -> CstStatLocal {
  CstStatLocal::new(
    vars_annotation_colon_positions,
    vars_comma_positions,
    values_comma_positions,
  )
}
