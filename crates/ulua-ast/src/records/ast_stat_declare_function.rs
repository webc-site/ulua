//! Faithful port of Luau `AstStatDeclareFunction : AstStat`
//! (`Ast/include/Luau/Ast.h`). Hand-ported (false-blocked via the bare-name
//! `AstAttr::Type` resolution). The two constructors and the
//! `visit`/`isCheckedFunction`/`has_attribute`/`get_attribute` methods are
//! separate items.

use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_name::AstName, ast_stat::AstStat,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack, location::Location,
  },
  rtti::{AstNodeClass, ast_rtti_index},
  type_aliases::ast_argument_name::AstArgumentName,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatDeclareFunction {
  pub base: AstStat,
  pub attributes: AstArray<*mut AstAttr>,
  pub name: AstName,
  pub name_location: Location,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  pub params: AstTypeList,
  pub param_names: AstArray<AstArgumentName>,
  pub vararg: bool,
  pub vararg_location: Location,
  pub ret_types: *mut AstTypePack,
}

impl AstNodeClass for AstStatDeclareFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("AstStatDeclareFunction");
}
