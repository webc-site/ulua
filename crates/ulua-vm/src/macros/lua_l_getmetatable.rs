use core::ffi::c_char;

use crate::{
  functions::lua_getfield::lua_getfield, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `LuaState`.
/// `n` must be a valid pointer to a null-terminated C string.
#[inline(always)]
pub unsafe fn lua_l_getmetatable(l: *mut LuaState, n: *const c_char) -> i32 {
  // C 宏 `luaL_getmetatable(L,n)` 即 `lua_getfield(L, LUA_REGISTRYINDEX, n)`。
  // lua_getfield 已是完整实现，直接调用（原 transmute 是为绕开早期 stub 而留，已无必要）
  // Safety: 契约保证 `l` 存活可压栈、`n` 为 NUL 结尾字符串，getfield 按注册表名取元表
  unsafe { lua_getfield(l, LUA_REGISTRYINDEX, n) }
}
