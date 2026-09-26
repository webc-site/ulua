use ulua_common::clock_shim::monotonic_seconds;

use crate::{functions::lua_pushnumber::lua_pushnumber, records::lua_state::LuaState};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn os_clock(l: *mut LuaState) -> i32 {
  unsafe {
    lua_pushnumber(l, monotonic_seconds());
    1
  }
}
