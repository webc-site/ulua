use ulua_analysis::{
  functions::simplify_intersection_simplify::simplify_intersection, records::arena_handle::Handle,
  type_aliases::type_id::TypeId,
};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn intersect(&mut self, a: TypeId, b: TypeId) -> TypeId {
    // Re-validate the (self-referential, move-stale) builtin_types pointer
    // before use — see `Fixture::get_builtins`.
    let builtin_types = self.base.get_builtins() as *mut _;
    simplify_intersection(
      Handle::from_ptr(builtin_types),
      Handle::from_mut(&mut self.arena),
      a,
      b,
    )
    .result
  }
}
