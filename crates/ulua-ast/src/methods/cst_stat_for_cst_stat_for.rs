use crate::{
  records::{cst_node::CstNode, cst_stat_for::CstStatFor, position::Position},
  rtti::CstNodeClass,
};

impl CstStatFor {
  pub fn new(
    annotation_colon_position: Position,
    equals_position: Position,
    end_comma_position: Position,
    step_comma_position: Position,
  ) -> Self {
    Self {
      base: CstNode::new(Self::CLASS_INDEX),
      annotation_colon_position,
      equals_position,
      end_comma_position,
      step_comma_position,
    }
  }
}

pub fn cst_stat_for_cst_stat_for(
  annotation_colon_position: Position,
  equals_position: Position,
  end_comma_position: Position,
  step_comma_position: Position,
) -> CstStatFor {
  CstStatFor::new(
    annotation_colon_position,
    equals_position,
    end_comma_position,
    step_comma_position,
  )
}
