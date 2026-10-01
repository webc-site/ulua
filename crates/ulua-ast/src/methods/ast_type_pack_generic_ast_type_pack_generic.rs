use crate::{
  records::{
    ast_name::AstName, ast_type_pack::AstTypePack, ast_type_pack_generic::AstTypePackGeneric,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypePackGeneric {
  pub fn new(location: Location, name: AstName) -> Self {
    Self {
      base: AstTypePack::new(<Self as AstNodeClass>::CLASS_INDEX, location),
      generic_name: name,
    }
  }
}
