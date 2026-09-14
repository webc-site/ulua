use core::marker::PhantomData;

use ulua_common::records::f_value::{FValue, FValueOverridable};

use crate::records::scoped_f_value::ScopedFValue;
impl<T: FValueOverridable> ScopedFValue<T> {
  pub fn scoped_f_value_luau_f_value_t_t(fvalue: &mut FValue<T>, new_value: T) -> Self {
    let old_value = fvalue.get();
    fvalue.push_test_override(new_value);

    ScopedFValue {
      value: fvalue as *mut FValue<T>,
      old_value,
      _marker: PhantomData,
    }
  }

  /// Idiomatic alias for the C++ `ScopedFValue(FValue<T>&, T)` ctor. Takes a
  /// shared `&FValue<T>` (the flag's interior mutability handles the write), so
  /// `ScopedFastFlag::new(&FFlag::SomeFlag, true)` binds against the `'static`
  /// flag instances — the spelling the test bodies use.
  pub fn new(fvalue: &FValue<T>, new_value: T) -> Self {
    let old_value = fvalue.get();
    fvalue.push_test_override(new_value);

    ScopedFValue {
      value: fvalue as *const FValue<T> as *mut FValue<T>,
      old_value,
      _marker: PhantomData,
    }
  }
}
