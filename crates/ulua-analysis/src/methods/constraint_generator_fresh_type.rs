//! @interface-stub
use crate::{
  enums::polarity::Polarity,
  functions::fresh_type::fresh_type,
  records::constraint_generator::ConstraintGenerator,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl ConstraintGenerator {
  pub fn fresh_type(&mut self, scope: &ScopePtr, polarity: Polarity) -> TypeId {
    let ft = unsafe {
      fresh_type(
        &mut *self.arena,
        &*self.builtin_types,
        scope.as_ref() as *const _ as *mut _,
        polarity,
      )
    };

    if let Some(interior_free_types) = self.interior_free_types.last_mut() {
      interior_free_types.types.push(ft);
    }

    self.free_types.insert_type_id(ft);
    ft
  }
}
