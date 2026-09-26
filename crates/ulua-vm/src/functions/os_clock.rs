use ulua_common::clock_shim::monotonic_seconds;

use crate::{
  functions::lua_pushnumber::lua_pushnumber, macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn os_clock(l: *mut LuaState) -> i32 {
  unsafe {
    lua_pushnumber(l, monotonic_seconds());
    1
  }
}

lua_lib_fn!(pub(crate) fn os_clock, os_clock_arm);
