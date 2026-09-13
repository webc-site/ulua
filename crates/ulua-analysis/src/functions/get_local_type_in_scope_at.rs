//! Node: `cxx:Function:Luau.Analysis:Analysis/src/AutocompleteCore.cpp:791:get_local_type_in_scope_at`
//! Source: `Analysis/src/AutocompleteCore.cpp:791-803` (hand-ported)

use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_local::AstExprLocal, ast_local::AstLocal, ast_node::AstNode,
    position::Position,
  },
  rtti::ast_node_as,
};

use crate::{
  functions::can_suggest_inferred_type_autocomplete_core::can_suggest_inferred_type,
  records::module::Module,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

pub fn get_local_type_in_scope_at(
  module: &Module,
  scope_at_position: &ScopePtr,
  position: Position,
  local: *mut AstLocal,
) -> Option<TypeId> {
  let _ = position;
  let mut binding_type = None;

  for (name, binding) in &scope_at_position.bindings {
    if name.local == local {
      binding_type = Some(binding.type_id);
      break;
    }
  }

  if let Some(ty) = binding_type
    && can_suggest_inferred_type(ty)
  {
    return Some(ty);
  }

  for (expr, expected_type) in module.ast_expected_types.iter() {
    let expr = *expr as *mut AstExpr;
    if expr.is_null() {
      continue;
    }

    let local_expr = unsafe { ast_node_as::<AstExprLocal>(expr as *mut AstNode) };
    if !local_expr.is_null() && unsafe { (*local_expr).local == local } {
      return Some(*expected_type);
    }
  }

  binding_type
}
