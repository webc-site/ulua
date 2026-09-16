use crate::records::{ast_node::AstNode, location::Location};

impl AstNode {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      class_index,
      location,
    }
  }
}

pub fn ast_node_ast_node(class_index: i32, location: Location) -> AstNode {
  AstNode::new(class_index, location)
}
