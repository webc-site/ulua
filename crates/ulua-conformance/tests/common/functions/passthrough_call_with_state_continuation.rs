use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  functions::{lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger},
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn passthrough_call_with_state_continuation(
  l: *mut lua_State,
  _status: c_int,
) -> c_int {
  unsafe {
    LUAU_ASSERT!(lua_l_checkinteger(l, 1) == 42);

    lua_gettop(l) - 1
  }
}
