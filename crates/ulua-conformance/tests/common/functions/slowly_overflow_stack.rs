use ulua_vm::{
  functions::{lua_l_checkstack::lua_l_checkstack, lua_pushnumber::lua_pushnumber},
  macros::luai_maxcstack::LUAI_MAXCSTACK,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn slowly_overflow_stack(l: *mut lua_State) -> i32 {
  for _ in 0..(LUAI_MAXCSTACK * 2) {
    unsafe {
      lua_l_checkstack(l, 1, "test");
      lua_pushnumber(l, 1.0);
    }
  }
  0
}
