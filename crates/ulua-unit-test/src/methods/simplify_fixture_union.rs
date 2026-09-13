use ulua_analysis::{functions::simplify_union::simplify_union, type_aliases::type_id::TypeId};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn union_(&mut self, a: TypeId, b: TypeId) -> TypeId {
    // Re-validate the (self-referential, move-stale) builtin_types pointer
    // before use — see `Fixture::get_builtins`.
    let builtin_types = self.base.get_builtins() as *mut _;
    simplify_union(builtin_types, &mut self.arena, a, b).result
  }
}
