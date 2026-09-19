use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_s_newlstr::luaS_newlstr,
  records::{lua_state::lua_State, t_string::tstring},
};

/// C++ `luaS_new` 宏（lstring.h）：取 C 字符串长度后走 `luaS_newlstr`。
/// 与 `lua_s_newliteral`（lstring.h 的另一宏镜像）实现相同，后者保留于独立模块。
///
/// # Safety
///
/// `l` 必须指向存活的 `lua_State`；`s` 必须指向 NUL 结尾缓冲区。
pub unsafe fn lua_s_new(l: *mut lua_State, s: *const c_char) -> *mut tstring {
  unsafe {
    let len = CStr::from_ptr(s).to_bytes().len();
    luaS_newlstr(l, s, len)
  }
}

pub use lua_s_new as luaS_new;
