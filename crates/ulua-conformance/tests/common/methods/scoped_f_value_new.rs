use core::marker::PhantomData;

use ulua_common::records::f_value::{FValue, FValueOverridable};

use crate::common::records::scoped_f_value::ScopedFValue;
impl<T: FValueOverridable> ScopedFValue<T> {
  pub fn new(fvalue: &FValue<T>, new_value: T) -> Self {
    fvalue.push_test_override(new_value);

    ScopedFValue {
      value: fvalue as *const FValue<T> as *mut FValue<T>,
      _marker: PhantomData,
    }
  }
}
