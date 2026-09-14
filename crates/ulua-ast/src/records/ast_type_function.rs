//! Faithful port of Luau `AstTypeFunction : AstType` (`Ast/include/Luau/Ast.h`).
//! Hand-ported (false-blocked via the bare-name `AstAttr::Type` resolution).
//! `AstArray<std::optional<AstArgumentName>>` -> `AstArray<Option<AstArgumentName>>`.
//! The two constructors and the `visit`/`isCheckedFunction`/`has_attribute`/
//! `get_attribute` methods are separate items.

use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_type::AstType, ast_type_list::AstTypeList,
    ast_type_pack::AstTypePack,
  },
  rtti::{AstNodeClass, ast_rtti_index},
  type_aliases::ast_argument_name::AstArgumentName,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeFunction {
  pub base: AstType,
  pub attributes: AstArray<*mut AstAttr>,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  pub arg_types: AstTypeList,
  pub arg_names: AstArray<Option<AstArgumentName>>,
  pub return_types: *mut AstTypePack,
}

impl AstNodeClass for AstTypeFunction {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeFunction");
}
