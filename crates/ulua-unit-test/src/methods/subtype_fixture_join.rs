use alloc::vec;

use ulua_analysis::{records::union_type::UnionType, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn join(&mut self, a: TypeId, b: TypeId) -> TypeId {
    self.arena.add_type(UnionType {
      options: vec![a, b],
    })
  }
}
