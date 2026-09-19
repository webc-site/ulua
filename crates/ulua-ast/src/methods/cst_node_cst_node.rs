use crate::records::cst_node::CstNode;

impl CstNode {
  pub const fn new(class_index: i32) -> Self {
    Self { class_index }
  }
}
