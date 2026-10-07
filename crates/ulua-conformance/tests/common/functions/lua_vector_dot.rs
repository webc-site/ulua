use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::safe_api::{l_checkvector, state_mut};

/// C-unwind ABI：作为 LuaCFunction 注册进 VM，避免 transmute。
pub(crate) extern "C-unwind" fn lua_vector_dot(l: *mut LuaState) -> c_int {
  let a = l_checkvector(l, 1);
  let b = l_checkvector(l, 2);

  let result = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  state_mut(l).push_number(result as f64);
  1
}
