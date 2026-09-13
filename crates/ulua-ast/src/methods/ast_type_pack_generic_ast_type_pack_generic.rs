use crate::{
  records::{
    ast_name::AstName, ast_node::AstNode, ast_type_pack::AstTypePack,
    ast_type_pack_generic::AstTypePackGeneric, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypePackGeneric {
  pub fn new(location: Location, name: AstName) -> Self {
    Self {
      base: AstTypePack {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      generic_name: name,
    }
  }
}

pub fn ast_type_pack_generic_ast_type_pack_generic(
  location: Location,
  name: AstName,
) -> AstTypePackGeneric {
  AstTypePackGeneric::new(location, name)
}
