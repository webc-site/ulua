use ulua_ast::records::position::Position;

use crate::{records::module::Module, type_aliases::scope_ptr_type::ScopePtr};

/// `ScopePtr findScopeAtPosition(const Module& module, Position pos)`.
/// Reference: `AstQuery.cpp`. `ScopePtr = shared_ptr<Scope>` (nullable) -> `Option<ScopePtr>`.
pub fn find_scope_at_position(module: &Module, pos: Position) -> Option<ScopePtr> {
  if module.scopes.is_empty() {
    return None;
  }

  let mut scope_location = module.scopes[0].0;
  let mut scope: Option<ScopePtr> = Some(module.scopes[0].1.clone());

  for &(loc, ref s) in &module.scopes {
    if loc.contains(pos) && (scope.is_none() || scope_location.encloses(&loc)) {
      scope_location = loc;
      scope = Some(s.clone());
    }
  }

  scope
}
