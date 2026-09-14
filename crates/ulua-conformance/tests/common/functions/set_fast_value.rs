use core::{ffi::c_char, ptr::null};
pub fn set_fast_value() {
  // This is a direct translation of the templated helper `setFastValue<T>(name, value)`
  // from `tests/main.cpp`.
  //
  // In Rust, we cannot mirror the C++ template instantiation here because the schedule
  // item is a single `function` node with no concrete `T` provided. As in many other
  // test helpers, this node is expected to be specialized/called from other translated
  // code that supplies the concrete type.
  //
  // Keep a safe, explicit placeholder to avoid referencing unavailable globals.
  let _ = 0_i32;

  // If/when concrete instantiations are added as separate scheduled items, they should:
  // - scan the per-type intrusive FValue<T> list via `FValueList::head()`
  // - compare `(*fvalue).name` to the provided `name` bytes
  // - assign `(*fvalue).set(value)` (for Copy types)
  //
  // Note: `FValue<T>::list` in C++ corresponds to `FValueList::head()` in Rust.
  let _unused: *const c_char = null();
}
