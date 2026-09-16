use ulua_vm::{functions::lua_pushnumber::lua_pushnumber, records::lua_state::lua_State};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn single_yield_continuation(l: *mut lua_State, _status: i32) -> i32 {
  unsafe {
    lua_pushnumber(l, 4.0);
  }
  1
}
