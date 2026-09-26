use alloc::string::ToString;

use ulua_analysis::{
  records::generic_type_pack::GenericTypePack, type_aliases::type_pack_id::TypePackId,
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn generic_pack(&mut self, name: &str) -> TypePackId {
    self
      .arena
      .add_type_pack_t(GenericTypePack::new_name(name.to_string()))
  }
}
