use ulua_analysis::type_aliases::{type_id::TypeId, type_pack_id::TypePackId};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn pack_initializer_list_type_id(&mut self, tys: Vec<TypeId>) -> TypePackId {
    self.arena.add_type_pack_initializer_list_type_id(&tys)
  }
}
