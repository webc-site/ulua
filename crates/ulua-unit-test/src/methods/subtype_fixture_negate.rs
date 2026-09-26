use ulua_analysis::{records::negation_type::NegationType, type_aliases::type_id::TypeId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn negate(&mut self, ty: TypeId) -> TypeId {
    self.arena.add_type(NegationType::new(ty))
  }
}
