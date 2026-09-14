use crate::records::{ast_node::AstNode, ast_type::AstType, location::Location};

impl AstType {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      base: AstNode {
        class_index,
        location,
      },
    }
  }
}

pub fn ast_type_ast_type(class_index: i32, location: Location) -> AstType {
  AstType::new(class_index, location)
}
