use core::ptr::NonNull;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_type::fresh_type,
  records::{scope::Scope, unifier_2::Unifier2},
  type_aliases::type_id::TypeId,
};

impl Unifier2 {
  pub fn fresh_type(&mut self, scope: NonNull<Scope>, polarity: Polarity) -> TypeId {
    let result = unsafe {
      fresh_type(
        &mut *self.arena.as_ptr(),
        &*self.builtin_types.as_ptr(),
        scope.as_ptr(),
        polarity,
      )
    };
    self.new_fresh_types.push(result);
    result
  }
}
