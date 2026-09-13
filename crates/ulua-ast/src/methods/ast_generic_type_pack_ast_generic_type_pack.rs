use crate::{
  records::{
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_node::AstNode,
    ast_type_pack::AstTypePack, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstGenericTypePack {
  pub fn new(location: Location, name: AstName, default_value: *mut AstTypePack) -> Self {
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

pub fn ast_generic_type_pack_ast_generic_type_pack(
  location: Location,
  name: AstName,
  default_value: *mut AstTypePack,
) -> AstGenericTypePack {
  AstGenericTypePack::new(location, name, default_value)
}
