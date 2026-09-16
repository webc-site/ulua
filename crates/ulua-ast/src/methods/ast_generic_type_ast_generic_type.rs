use crate::{
  records::{
    ast_generic_type::AstGenericType, ast_name::AstName, ast_node::AstNode, ast_type::AstType,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstGenericType {
  pub fn new(location: Location, name: AstName, default_value: *mut AstType) -> Self {
    Self {
      base: AstNode {
        class_index: <Self as AstNodeClass>::CLASS_INDEX,
        location,
      },
      name,
      default_value,
    }
  }
}

pub fn ast_generic_type_ast_generic_type(
  location: Location,
  name: AstName,
  default_value: *mut AstType,
) -> AstGenericType {
  AstGenericType::new(location, name, default_value)
}
