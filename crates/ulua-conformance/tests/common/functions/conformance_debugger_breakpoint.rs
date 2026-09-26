use core::{ffi::c_int, mem::zeroed};

use ulua_vm::{
  functions::{
    lua_breakpoint::lua_breakpoint, lua_getinfo::lua_getinfo,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_optboolean::lua_l_optboolean,
    lua_stackdepth::lua_stackdepth,
  },
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::functions::cstr::cstr;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_breakpoint(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    let line = lua_l_checkinteger(l, 1);
    let enabled = lua_l_optboolean(l, 2, true);

    let mut ar: LuaDebug = zeroed();
    lua_getinfo(l, lua_stackdepth(&*l) - 1, cstr(b"f\0"), &mut ar);

    lua_breakpoint(l, -1, line, if enabled { 1 } else { 0 });
    0
  }
}
