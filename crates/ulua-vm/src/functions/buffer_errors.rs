//! buffer 族共享错误消息（cpp lbuffer.cpp/lstrlib.cpp 同款字面量）。
//! format_args! 首参必须是字符串字面量，无法用 &str 常量传递，故用函数收口。

use crate::{macros::lua_l_error::luaL_error, type_aliases::lua_state::lua_State};

/// "buffer access out of bounds" 错误（必然抛出，不返回）
///
/// # Safety
/// `l` 必须是有效且存活的 `lua_State` 指针。
#[inline]
pub(crate) unsafe fn buffer_oob_error(l: *mut lua_State) -> ! {
  unsafe {
    luaL_error!(l, "buffer access out of bounds");
  }
}

/// "bit count is out of range of [0; 32]" 错误（必然抛出，不返回）
///
/// # Safety
/// `l` 必须是有效且存活的 `lua_State` 指针。
#[inline]
pub(crate) unsafe fn buffer_bitcount_error(l: *mut lua_State) -> ! {
  unsafe {
    luaL_error!(l, "bit count is out of range of [0; 32]");
  }
}
