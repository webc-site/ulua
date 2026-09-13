use core::marker::PhantomData;

use ulua_common::records::f_value::{FValue, FValueOverridable};

#[derive(Debug)]
pub struct ScopedFValue<T: FValueOverridable> {
  pub(crate) value: *mut FValue<T>,
  pub(crate) _marker: PhantomData<T>,
}
