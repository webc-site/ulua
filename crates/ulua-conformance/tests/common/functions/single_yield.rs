use ulua_vm::{
  functions::{lua_pushnumber::lua_pushnumber, lua_yield::lua_yield},
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn single_yield(l: *mut lua_State) -> i32 {
  unsafe {
    lua_pushnumber(l, 2.0);
    lua_yield(l, 1)
  }
}
