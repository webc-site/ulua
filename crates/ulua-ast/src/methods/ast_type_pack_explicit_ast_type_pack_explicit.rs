use crate::{
  records::{
    ast_node::AstNode, ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstTypePackExplicit {
  pub fn new(location: Location, type_list: AstTypeList) -> Self {
    Self {
      base: AstTypePack {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
      },
      type_list,
    }
  }
}

pub fn ast_type_pack_explicit_ast_type_pack_explicit(
  location: Location,
  type_list: AstTypeList,
) -> AstTypePackExplicit {
  AstTypePackExplicit::new(location, type_list)
}
