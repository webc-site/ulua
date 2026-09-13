use crate::{
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_node::AstNode,
    ast_stat::AstStat, ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType,
    location::Location,
  },
  rtti::AstNodeClass,
};

impl AstStatTypeAlias {
  pub fn new_simple(
    location: Location,
    name: AstName,
    name_location: Location,
    generics: AstArray<*mut AstGenericType>,
    generic_packs: AstArray<*mut AstGenericTypePack>,
    type_: *mut AstType,
    exported: bool,
  ) -> Self {
    Self {
      base: AstStat {
        base: AstNode {
          class_index: Self::CLASS_INDEX,
          location,
        },
        has_semicolon: false,
      },
      name,
      name_location,
      generics,
      generic_packs,
      type_ptr: type_,
      exported,
    }
  }
}

pub fn ast_stat_type_alias_ast_stat_type_alias(
  location: Location,
  name: AstName,
  name_location: Location,
  generics: AstArray<*mut AstGenericType>,
  generic_packs: AstArray<*mut AstGenericTypePack>,
  type_: *mut AstType,
  exported: bool,
) -> AstStatTypeAlias {
  AstStatTypeAlias::new_simple(
    location,
    name,
    name_location,
    generics,
    generic_packs,
    type_,
    exported,
  )
}
