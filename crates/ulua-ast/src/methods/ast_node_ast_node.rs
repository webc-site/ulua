use crate::records::{ast_node::AstNode, location::Location};

impl AstNode {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      class_index,
      location,
    }
  }
}
