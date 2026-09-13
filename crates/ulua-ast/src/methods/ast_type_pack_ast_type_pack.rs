use crate::records::{ast_node::AstNode, ast_type_pack::AstTypePack, location::Location};

impl AstTypePack {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      base: AstNode {
        class_index,
        location,
      },
    }
  }
}
