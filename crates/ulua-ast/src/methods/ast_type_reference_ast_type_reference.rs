use crate::{
  records::{
    ast_array::AstArray, ast_name::AstName, ast_node::AstNode, ast_type::AstType,
    ast_type_or_pack::AstTypeOrPack, ast_type_reference::AstTypeReference, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypeReference {
  pub fn new(
    location: Location,
    prefix: Option<AstName>,
    name: AstName,
    prefix_location: Option<Location>,
    name_location: Location,
    has_parameter_list: bool,
    parameters: AstArray<AstTypeOrPack>,
  ) -> Self {
    Self {
      base: AstType {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      has_parameter_list,
      prefix,
      prefix_location,
      name,
      name_location,
      parameters,
    }
  }
}

pub fn ast_type_reference_ast_type_reference(
  location: Location,
  prefix: Option<AstName>,
  name: AstName,
  prefix_location: Option<Location>,
  name_location: Location,
  has_parameter_list: bool,
  parameters: AstArray<AstTypeOrPack>,
) -> AstTypeReference {
  AstTypeReference::new(
    location,
    prefix,
    name,
    prefix_location,
    name_location,
    has_parameter_list,
    parameters,
  )
}
