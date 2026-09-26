use crate::{
  records::{
    ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_or_pack::AstTypeOrPack,
    ast_type_reference::AstTypeReference, location::Location,
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
      base: AstType::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      has_parameter_list,
      prefix,
      prefix_location,
      name,
      name_location,
      parameters,
    }
  }
}
