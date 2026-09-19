use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeError {
  pub fn new(
    location: Location,
    types: AstArray<*mut AstType>,
    is_missing: bool,
    message_index: u32,
  ) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      types,
      is_missing,
      message_index,
    }
  }
}
