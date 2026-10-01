#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstStatTypeAlias {
  pub base: AstStat,
  pub name: AstName,
  pub name_location: Location,
  pub generics: AstArray<*mut AstGenericType>,
  pub generic_packs: AstArray<*mut AstGenericTypePack>,
  /// cpp `AstType* type`（`Ast.h:1009`）：唯一构造点 `Parser.cpp:1483` 以
  /// `parseTypeAnnotation` 结果接线（并以 `type->location` 定界），文法必建，恒非空。
  pub type_ptr: Node<AstType>,
  pub exported: bool,
}

use crate::records::{
  ast_array::AstArray, ast_generic_type::AstGenericType, ast_generic_type_pack::AstGenericTypePack,
  ast_name::AstName, ast_stat::AstStat, ast_type::AstType, location::Location, node_handle::Node,
};
