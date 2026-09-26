use crate::records::{cst_node::CstNode, position::Position};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CstStatFor {
  pub base: CstNode,
  pub annotation_colon_position: Position,
  pub equals_position: Position,
  pub end_comma_position: Position,
  pub step_comma_position: Position,
}

impl_cst_node_class!(CstStatFor);
impl_cst_node_new!(
  CstStatFor,
  annotation_colon_position: Position,
  equals_position: Position,
  end_comma_position: Position,
  step_comma_position: Position,
);
