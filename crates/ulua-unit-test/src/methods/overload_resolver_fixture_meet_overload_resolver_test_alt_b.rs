use ulua_analysis::{records::intersection_type::IntersectionType, type_aliases::type_id::TypeId};

use crate::records::overload_resolver_fixture::OverloadResolverFixture;

impl OverloadResolverFixture {
  pub fn meet_initializer_list_type_id(&self, parts: &[TypeId]) -> TypeId {
    let intersection = IntersectionType {
      parts: parts.to_vec(),
    };
    unsafe { (*self.arena).add_type(intersection) }
  }
}
