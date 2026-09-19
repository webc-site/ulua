use core::ffi::{CStr, c_char};

use crate::{functions::lua_pushlstring::lua_pushlstring, type_aliases::lua_state::lua_State};

/// cpp `lua.h:517` `#define lua_pushliteral(L, s) lua_pushlstring(L, "" s, len)` 对应：
/// 长度即 C 字符串字节数，直接转发 `lua_pushlstring`，无返回值。
///
/// # Safety
///
/// `l` 必须指向存活的 `lua_State`；`s` 必须指向 NUL 结尾的 C 字符串。
#[inline(always)]
pub unsafe fn lua_pushliteral(l: *mut lua_State, s: *const c_char) {
  unsafe {
    let len = CStr::from_ptr(s).to_bytes().len();
    lua_pushlstring(l, s, len);
  }
}
