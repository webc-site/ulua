use crate::{
  records::{
    ast_array::AstArray, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_stat::AstStat,
    ast_stat_type_alias::AstStatTypeAlias, ast_type::AstType, location::Location,
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
      base: AstStat::new(Self::CLASS_INDEX, location),
      name,
      name_location,
      generics,
      generic_packs,
      type_ptr: type_,
      exported,
    }
  }
}
