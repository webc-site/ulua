use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::state_ref;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_api_calls_check_not_yieldable(
  l: *mut LuaState,
) -> c_int {
  assert!(!state_ref(l).is_yieldable());
  0
}
