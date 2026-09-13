use ulua_ast::records::position::Position;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::type_aliases::{module_ptr_module::ModulePtr, scope_ptr_type::ScopePtr};

pub fn find_closest_scope(module: &ModulePtr, scope_pos: &Position) -> ScopePtr {
  LUAU_ASSERT!(module.has_module_scope());
  let mut closest: ScopePtr = module.get_module_scope();
  // find the scope the nearest statement belonged to.
  for (_loc, sc) in &module.scopes {
    // We bias towards the later scopes because those correspond to inner scopes.
    // in the case of if statements, we create two scopes at the same location for the body of the then
    // and else branches, so we need to bias later. This is why the closest update condition has a <=
    // instead of a <
    if sc.location.contains(*scope_pos) && closest.location.begin <= sc.location.begin {
      closest = sc.clone();
    }
  }
  closest
}
