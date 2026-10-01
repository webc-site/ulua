use ulua_vm::{
  records::lua_state::LuaState,
  type_aliases::{lua_c_function::LuaCFunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  c_yielding_iterator::c_yielding_iterator,
  c_yielding_iterator_continuation::c_yielding_iterator_continuation,
  safe_api::{pushcclosurek, state_mut}, setup_native_helpers::setup_native_helpers,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_iter_setup(l: *mut LuaState) {
  // Safety: `l` 存活（本入口的 C ABI 契约转承），setup_native_helpers 内部全走 safe 门面。
  unsafe { setup_native_helpers(l) };

  let iterator: LuaCFunction = Some(c_yielding_iterator);
  let continuation: LuaContinuation = Some(c_yielding_iterator_continuation);

  pushcclosurek(
    l,
    iterator,
    Some(b"cYieldingIterator\0"),
    0,
    continuation,
  );
  state_mut(l).set_global_str("cYieldingIterator");
}
