//! Source: `Analysis/src/AutocompleteCore.cpp:964-976`

use ulua_ast::records::{ast_local::AstLocal, position::Position};

use crate::{
  records::module::Module,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

/// Upstream reads neither `module` nor `position`; they stay in the signature for
/// parity with `getLocalTypeInScopeAt(const Module&, const ScopePtr&, Position, AstLocal*)`.
pub fn get_local_type_in_scope_at(
  _module: &Module,
  scope_at_position: &ScopePtr,
  _position: Position,
  local: *mut AstLocal,
) -> Option<TypeId> {
  scope_at_position
    .bindings
    .iter()
    .find(|(name, _)| name.local == local)
    .map(|(_, binding)| binding.type_id)
}
