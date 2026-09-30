use core::marker::PhantomData;

use ulua_common::records::f_value::{FValue, FValueOverridable};

/// Test RAII guard — port of `tests/ScopedFlags.h`'s `ScopedFValue<T>`. Installs
/// a THREAD-LOCAL override of the flag on construction and removes it on drop
/// (the `Drop` impl lives alongside the ctors in `methods/`). Unlike C++ (which
/// mutates the process-global flag — safe only because doctest is
/// single-threaded), the override is private to the constructing thread, so
/// parallel libtest threads don't interfere.
/// `ScopedFastFlag = ScopedFValue<bool>`, `ScopedFastInt = ScopedFValue<int>`.
///
/// Move-only and `Drop`-bearing (no `Copy`/`Clone`), matching the C++ `= delete`d
/// copy ctor.
#[derive(Debug)]
pub struct ScopedFValue<T: FValueOverridable> {
  pub value: *mut FValue<T>,
  pub old_value: T,
  pub _marker: PhantomData<T>,
}
