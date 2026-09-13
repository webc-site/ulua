//! `FValue<T>::operator T() const` — reads the flag's current value. Reference:
//! `luau/Common/include/Luau/Common.h` (`LUAU_FORCEINLINE operator T()`).
//!
//! Modelled as `get()` rather than a `From`/`Deref` conversion: the value lives
//! behind `UnsafeCell` (so a `static` flag stays mutable), so it is returned by
//! copy, not by reference.

use crate::records::f_value::{FValue, FValueOverridable, overrides_active};

impl<T: FValueOverridable> FValue<T> {
  pub fn get(&self) -> T {
    // Fast path: no test has ever installed an override -> read the global.
    // A test that installs a `ScopedFastFlag`/`ScopedFastInt` flips
    // `OVERRIDES_ACTIVE`, and its per-thread override then shadows the
    // global for that thread only (so parallel tests don't interfere).
    if overrides_active()
      && let Some(v) = T::override_top(self as *const FValue<T> as usize)
    {
      return v;
    }
    unsafe { *self.value.get() }
  }
}
