#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatTypeAlias {
  pub base: AstStat,
  pub name: AstName,
  pub name_location: Location,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  pub type_ptr: *mut AstType,
  pub exported: bool,
}

use crate::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName, ast_stat::AstStat, ast_type::AstType, location::Location,
};
