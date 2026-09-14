use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::lua_gettop::lua_gettop, macros::lua_tonumber::lua_tonumber,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    LUAU_ASSERT!(lua_gettop(l) == 4);
    LUAU_ASSERT!(lua_tonumber!(l, -1) == 0.5);
    1
  }
}
