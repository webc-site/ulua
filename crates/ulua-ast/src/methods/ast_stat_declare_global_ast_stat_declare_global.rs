use crate::{
  records::{
    ast_name::AstName, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_type::AstType, location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatDeclareGlobal {
  pub fn new(
    location: Location,
    name: AstName,
    name_location: Location,
    type_: *mut AstType,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: <Self as AstNodeClass>::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      name_location,
      type_,
    }
  }
}

pub fn ast_stat_declare_global_ast_stat_declare_global(
  location: Location,
  name: AstName,
  name_location: Location,
  type_: *mut AstType,
) -> AstStatDeclareGlobal {
  AstStatDeclareGlobal::new(location, name, name_location, type_)
}
