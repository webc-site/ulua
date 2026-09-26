use core::ffi::c_char;

use crate::{
  functions::lua_setfield::lua_setfield, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶已压入待设值；`s` 必须指向 NUL 结尾 C 字符串
/// （作为 globals 表字段名被 strlen 读取）。
#[inline(always)]
pub unsafe fn lua_setglobal(l: *mut LuaState, s: *const c_char) {
  // Safety: 契约保证 `l` 存活、栈顶有值、`s` 为 NUL 结尾字符串
  unsafe {
    lua_setfield(l, LUA_GLOBALSINDEX, s);
  }
}
