use crate::records::{
  ast_array::AstArray, ast_name::AstName, ast_type::AstType, ast_type_or_pack::AstTypeOrPack,
  location::Location,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeReference {
  pub base: AstType,
  pub has_parameter_list: bool,
  pub prefix: Option<AstName>,
  pub prefix_location: Option<Location>,
  pub name: AstName,
  pub name_location: Location,
  pub parameters: AstArray<AstTypeOrPack>,
}
