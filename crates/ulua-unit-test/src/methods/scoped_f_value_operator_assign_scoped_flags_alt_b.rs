use core::ptr::null_mut;

use ulua_common::records::f_value::FValueOverridable;

use crate::records::scoped_f_value::ScopedFValue;
impl<T: FValueOverridable> ScopedFValue<T> {
  pub fn operator_assign_mut(&mut self, mut rhs: ScopedFValue<T>) -> &mut Self {
    self.value = rhs.value;
    self.old_value = rhs.old_value;

    rhs.value = null_mut();

    self
  }
}
