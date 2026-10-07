use core::ffi::c_int;

use ulua_vm::{functions::lua_stackdepth::lua_stackdepth, records::lua_state::LuaState};

use crate::common::functions::safe_api::{
  breakpoint, getinfo, l_optboolean, state_mut, zero_debug,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_breakpoint(l: *mut LuaState) -> c_int {
  let line = state_mut(l).check_integer(1);
  let enabled = l_optboolean(l, 2, true);

  let mut ar = zero_debug();
  // `lua_stackdepth` 已是 ulua-vm 安全签名（`&LuaState`），经单点重建直调。
  getinfo(l, lua_stackdepth(state_mut(l)) - 1, b"f\0", &mut ar);

  breakpoint(l, -1, line, enabled as c_int);
  0
}
