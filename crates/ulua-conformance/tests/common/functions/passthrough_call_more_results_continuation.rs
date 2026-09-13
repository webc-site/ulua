use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_gettop::lua_gettop,
  macros::{lua_isnil::lua_isnil, lua_pop::lua_pop, lua_tonumber::lua_tonumber},
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_more_results_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    LUAU_ASSERT!(lua_gettop(l) == 13);

    for _ in 0..9 {
      LUAU_ASSERT!(lua_isnil!(l, -1));
      lua_pop(l, 1);
    }

    LUAU_ASSERT!(lua_tonumber!(l, -1) == 0.5);
    1
  }
}
