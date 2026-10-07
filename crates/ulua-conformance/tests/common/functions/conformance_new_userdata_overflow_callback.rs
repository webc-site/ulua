use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_new_userdata_overflow_dtor::conformance_new_userdata_overflow_dtor,
  safe_api::{newuserdatadtor, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_new_userdata_overflow_callback(
  l: *mut LuaState,
) -> c_int {
  // `usize::MAX` 尺寸分配按 cpp 预期走 OOM 失败路径（VM 收敛为 Lua 错误）。
  newuserdatadtor(l, usize::MAX, Some(conformance_new_userdata_overflow_dtor));
  state_mut(l).get_metatable(-1);

  0
}
