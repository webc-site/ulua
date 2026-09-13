use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn meet_type_id_type_id(&self, a: TypeId, b: TypeId) -> TypeId {
    self.meet_initializer_list_type_id(&[a, b])
  }
}
