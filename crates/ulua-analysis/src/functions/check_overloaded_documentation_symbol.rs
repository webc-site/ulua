use ulua_ast::{
  records::{ast_expr::AstExpr, ast_node::AstNode},
  rtti,
};

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    to_string_to_string_alt_c::to_string_type_id,
  },
  records::{intersection_type::IntersectionType, module::Module},
  type_aliases::{documentation_symbol::DocumentationSymbol, type_id::TypeId},
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_overloaded_documentation_symbol(
  module: &Module,
  ty: TypeId,
  parent_expr: *const AstExpr,
  documentation_symbol: Option<DocumentationSymbol>,
) -> Option<DocumentationSymbol> {
  let documentation_symbol = documentation_symbol?;

  let follow_ty = follow_type_id(ty);
  let intersection_ptr = get_type_id::<IntersectionType>(follow_ty);
  if intersection_ptr.is_none() {
    return Some(documentation_symbol);
  }

  let matching_overload = if !parent_expr.is_null()
    && unsafe { (*parent_expr).base.class_index == rtti::ast_rtti_index("AstExprCall") }
  {
    let node_ptr = parent_expr as *const AstNode;
    module.ast_overload_resolved_types.find(&node_ptr).copied()
  } else {
    None
  };

  if let Some(matching_overload) = matching_overload {
    let mut overload_symbol = documentation_symbol;
    overload_symbol.push_str("/overload/");
    let ty_str = to_string_type_id(matching_overload);
    overload_symbol.push_str(&ty_str);
    Some(overload_symbol)
  } else {
    Some(documentation_symbol)
  }
}
