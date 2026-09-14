use ulua_analysis::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn pack_vector_type_id(&self, tys: Vec<TypeId>) -> TypePackId {
    self.pack_initializer_list_type_id(&tys)
  }
}
