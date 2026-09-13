use core::ffi::c_char;

use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType,
    ast_type_singleton_string::AstTypeSingletonString, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeSingletonString {
  pub fn new(location: Location, value: AstArray<c_char>) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      value,
    }
  }
}

pub fn ast_type_singleton_string_ast_type_singleton_string(
  location: Location,
  value: AstArray<c_char>,
) -> AstTypeSingletonString {
  AstTypeSingletonString::new(location, value)
}
