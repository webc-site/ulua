//! Faithful port of Luau `AstExprFunction : AstExpr` (`Ast/include/Luau/Ast.h`).
//!
//! Hand-ported (false-blocked via the bare-name `AstAttr::Type` resolution).
//! `AstLocal* self` -> `self_` (Rust keyword). `std::optional<Location>` ->
//! `Option<Location>`. The two constructors and the
//! `has_native_attribute`/`has_attribute`/`get_attribute`/`visit` methods are
//! separate items.

use crate::records::{
  ast_attr::AstAttr,
  ast_expr::AstExpr,
  ast_generic_type::AstGenericType,
  ast_generic_type_pack::AstGenericTypePack,
  ast_local::AstLocal,
  ast_name::AstName,
  ast_stat_block::AstStatBlock,
  ast_type_pack::AstTypePack,
  location::Location,
  node_handle::{Node, Nodes, OptNode},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstExprFunction {
  pub base: AstExpr,
  pub attributes: Nodes<AstAttr>,
  pub generics: Nodes<AstGenericType>,
  pub generic_packs: Nodes<AstGenericTypePack>,
  pub self_: OptNode<AstLocal>,
  pub args: Nodes<AstLocal>,
  pub return_annotation: OptNode<AstTypePack>,
  pub vararg: bool,
  pub vararg_location: Location,
  pub vararg_annotation: OptNode<AstTypePack>,
  pub body: Node<AstStatBlock>,
  pub function_depth: usize,
  pub debugname: AstName,
  pub arg_location: Option<Location>,
}
