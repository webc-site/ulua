//! Faithful port of Luau `AstTypeFunction : AstType` (`Ast/include/Luau/Ast.h`).
//! Hand-ported (false-blocked via the bare-name `AstAttr::Type` resolution).
//! `AstArray<std::optional<AstArgumentName>>` -> `AstArray<Option<AstArgumentName>>`.
//! The two constructors and the `visit`/`isCheckedFunction`/
//! `get_attribute` methods are separate items.

use crate::{
  records::{
    ast_array::AstArray, ast_attr::AstAttr, ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack, ast_type::AstType, ast_type_list::AstTypeList,
    ast_type_pack::AstTypePack, node_handle::Node,
  },
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
  /// cpp `AstTypePack* returnTypes`（`Ast.h:1283`）：三处构造（`Parser.cpp:1797/3105`、
  /// `TypeAttach.cpp:379`）均接线 `parseReturnType` 或显式空 pack（declare 路径在
  /// `Parser.cpp:1843-1846` 对 `parseOptionalReturnType` 的 null 结果现场补建），恒非空。
  pub return_types: Node<AstTypePack>,
}
