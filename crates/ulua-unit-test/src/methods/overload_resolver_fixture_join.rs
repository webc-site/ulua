use ulua_analysis::{records::union_type::UnionType, type_aliases::type_id::TypeId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn join(&self, a: TypeId, b: TypeId) -> TypeId {
    unsafe {
      (*self.arena).add_type(UnionType {
        options: vec![a, b],
      })
    }
  }
}
