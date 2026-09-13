use core::{ffi::c_int, mem::zeroed};

use ulua_vm::{
  functions::{
    lua_breakpoint::lua_breakpoint, lua_getinfo::lua_getinfo,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_optboolean::lua_l_optboolean,
    lua_stackdepth::lua_stackdepth,
  },
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_breakpoint(l: *mut lua_State) -> c_int {
  unsafe {
    let line = lua_l_checkinteger(l, 1);
    let enabled = lua_l_optboolean(l, 2, true);

    let mut ar: LuaDebug = zeroed();
    lua_getinfo(l, lua_stackdepth(l) - 1, c"f".as_ptr(), &mut ar);

    lua_breakpoint(l, -1, line, if enabled { 1 } else { 0 });
    0
  }
}
