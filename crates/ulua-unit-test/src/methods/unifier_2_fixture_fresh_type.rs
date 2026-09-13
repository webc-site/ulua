//! @interface-stub
use ulua_analysis::{
  enums::polarity::Polarity, functions::get_mutable_type::get_mutable_type_id,
  records::free_type::FreeType, type_aliases::type_id::TypeId,
};

use crate::records::unifier_2_fixture::Unifier2Fixture;

impl Unifier2Fixture {
  pub fn fresh_type(&mut self) -> (TypeId, *mut FreeType) {
    let ty = self
      .arena
      .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
        &mut *self.scope,
        self.builtin_types.never_type,
        self.builtin_types.unknown_type,
        Polarity::Unknown,
      ));
    let free = get_mutable_type_id::<FreeType>(ty).expect("expected FreeType");
    (ty, free as *mut FreeType)
  }
}
