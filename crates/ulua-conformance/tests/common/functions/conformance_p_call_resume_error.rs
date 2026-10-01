use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{resumeerror, tothread, xmove};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_p_call_resume_error(l: *mut LuaState) -> c_int {
  let co = tothread(l, 1).expect("arg1 为 coroutine");
  xmove(l, co, 1);
  resumeerror(co, l);
  0
}
