use alloc::{string::ToString, sync::Arc};

use ulua_analysis::{
  records::{generic_type::GenericType, scope::Scope},
  type_aliases::type_id::TypeId,
};

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn generic(&mut self, name: &str) -> TypeId {
    let scope = Arc::as_ptr(&self.module_scope) as *mut Scope;
    self.arena.add_type(GenericType::generic_type_scope_name(
      scope,
      &name.to_string(),
    ))
  }
}
