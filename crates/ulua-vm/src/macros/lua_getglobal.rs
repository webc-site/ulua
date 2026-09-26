use core::ffi::c_char;

use crate::{
  functions::lua_getfield::lua_getfield, macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶已预留 1 槽承接结果；`s` 必须指向 NUL 结尾
/// C 字符串（`lua_getfield` 将其 strlen 后作为 globals 键查询）。
#[inline(always)]
pub unsafe fn lua_getglobal(l: *mut LuaState, s: *const c_char) -> i32 {
  // Safety: 契约保证 `l` 存活、`s` 为 NUL 结尾字符串，lua_getfield 按此读取并压栈
  unsafe { lua_getfield(l, LUA_GLOBALSINDEX, s) }
}
