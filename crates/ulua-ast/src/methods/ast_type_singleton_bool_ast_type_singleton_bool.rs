use crate::{
  records::{
    ast_node::AstNode, ast_type::AstType, ast_type_singleton_bool::AstTypeSingletonBool,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeSingletonBool {
  pub fn new(location: Location, value: bool) -> Self {
    AstTypeSingletonBool {
      base: AstType {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
      },
      value,
    }
  }
}
