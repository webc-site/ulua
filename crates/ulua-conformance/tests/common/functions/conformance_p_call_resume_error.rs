use core::ffi::c_int;

use ulua_vm::{
  functions::{lua_resumeerror::lua_resumeerror, lua_tothread::lua_tothread, lua_xmove::lua_xmove},
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_p_call_resume_error(l: *mut lua_State) -> c_int {
  unsafe {
    let co = lua_tothread(l, 1);
    lua_xmove(l, co, 1);
    lua_resumeerror(co, l);
    0
  }
}
