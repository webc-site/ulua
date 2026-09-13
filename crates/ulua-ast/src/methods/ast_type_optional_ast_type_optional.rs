use core::ptr::null_mut;

use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_optional::AstTypeOptional, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeOptional {
  pub fn new(location: Location) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      type_: null_mut(),
    }
  }
}

pub fn ast_type_optional_ast_type_optional(location: Location) -> AstTypeOptional {
  AstTypeOptional::new(location)
}
