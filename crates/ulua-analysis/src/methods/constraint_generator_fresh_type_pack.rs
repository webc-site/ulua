use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  records::{
    constraint_generator::ConstraintGenerator, free_type_pack::FreeTypePack, scope::Scope,
    type_level::TypeLevel,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl ConstraintGenerator {
  pub fn fresh_type_pack(&mut self, scope: &ScopePtr, polarity: Polarity) -> TypePackId {
    // FreeTypePack f{scope.get(), polarity};
    let mut free = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Polarity::None,
    };
    free.free_type_pack_scope_polarity(scope.as_ref() as *const Scope as *mut Scope, polarity);

    // arena->addTypePack(TypePackVar{std::move(f)})
    let result = unsafe { (*self.arena).add_type_pack_t(free) };

    // interiorFreeTypes.back().typePacks.push_back(result)
    if let Some(interior) = self.interior_free_types.last_mut() {
      interior.type_packs.push(result);
    }

    result
  }
}
