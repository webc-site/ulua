use std::sync::Arc;

use ulua_analysis::{
  enums::polarity::Polarity,
  functions::get_mutable_type::get_mutable_type_id,
  records::{free_type::FreeType, scope::Scope},
  type_aliases::type_id::TypeId,
};

use crate::records::generalization_fixture::GeneralizationFixture;

impl GeneralizationFixture {
  pub fn fresh_type(&mut self) -> (TypeId, *mut FreeType) {
    let scope = Arc::as_ptr(&self.scope) as *mut Scope;
    let ty = self
      .arena
      .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
        scope,
        self.builtin_types.never_type,
        self.builtin_types.unknown_type,
        Polarity::Unknown,
      ));
    let free = get_mutable_type_id::<FreeType>(ty).expect("expected FreeType");
    (ty, free as *mut FreeType)
  }
}
