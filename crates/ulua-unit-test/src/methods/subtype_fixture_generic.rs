use alloc::string::ToString;

use ulua_analysis::{records::generic_type::GenericType, type_aliases::type_id::TypeId};

use crate::{functions::raw_handle::raw_handle, records::subtype_fixture::SubtypeFixture};

impl SubtypeFixture {
  pub fn generic(&mut self, name: &str) -> TypeId {
    let scope = raw_handle(&self.module_scope);
    self.arena.add_type(GenericType::generic_type_scope_name(
      scope,
      &name.to_string(),
    ))
  }
}
