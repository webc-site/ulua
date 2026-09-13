use ulua_analysis::{records::negation_type::NegationType, type_aliases::type_id::TypeId};

use crate::records::simplify_fixture::SimplifyFixture;

impl SimplifyFixture {
  pub fn mk_negation(&mut self, ty: TypeId) -> TypeId {
    self.arena.add_type(NegationType::new(ty))
  }
}
