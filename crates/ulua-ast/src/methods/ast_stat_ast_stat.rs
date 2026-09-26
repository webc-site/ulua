use crate::records::{ast_node::AstNode, ast_stat::AstStat, location::Location};

impl AstStat {
  pub fn new(class_index: i32, location: Location) -> Self {
    Self {
      base: AstNode {
        class_index,
        location,
      },
      has_semicolon: false,
    }
  }
}
