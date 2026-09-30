use alloc::vec;

use ulua_analysis::{records::intersection_type::IntersectionType, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn meet(&mut self, a: TypeId, b: TypeId) -> TypeId {
    self.arena.add_type(IntersectionType { parts: vec![a, b] })
  }
}
