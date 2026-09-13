use crate::records::cst_node::CstNode;

impl CstNode {
  pub const fn new(class_index: i32) -> Self {
    Self { class_index }
  }
}

pub fn cst_node_cst_node(class_index: i32) -> CstNode {
  CstNode::new(class_index)
}
