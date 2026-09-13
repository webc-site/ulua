use ulua_common::records::f_value::FValueOverridable;

use crate::records::scoped_f_value::ScopedFValue;

impl<T: FValueOverridable> ScopedFValue<T> {
  pub fn operator_assign(&mut self, _rhs: &ScopedFValue<T>) -> &mut Self {
    // C++: ScopedFValue& operator=(const ScopedFValue&) = delete;
    // This overload is deleted in C++; in Rust we implement it as a no-op stub
    // that panics if ever called, preserving the "deleted" semantics at runtime.
    panic!("ScopedFValue assignment operator is deleted in C++");
  }
}
