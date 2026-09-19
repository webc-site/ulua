use core::ffi::{c_char, c_int};

use crate::{
  functions::lua_getfield::lua_getfield, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::lua_State,
};

/// # Safety
///
/// `l` must be a valid pointer to a live `lua_State`.
/// `n` must be a valid pointer to a null-terminated C string.
#[inline(always)]
pub unsafe fn lua_l_getmetatable(l: *mut lua_State, n: *const c_char) -> c_int {
  // C 宏 `luaL_getmetatable(L,n)` 即 `lua_getfield(L, LUA_REGISTRYINDEX, n)`。
  // lua_getfield 已是完整实现，直接调用（原 transmute 是为绕开早期 stub 而留，已无必要）
  unsafe { lua_getfield(l, LUA_REGISTRYINDEX, n) }
}
