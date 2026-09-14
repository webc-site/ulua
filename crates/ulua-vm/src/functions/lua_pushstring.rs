use core::ffi::c_char;

use crate::{
  functions::{lua_pushlstring::lua_pushlstring, lua_pushnil::lua_pushnil},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_pushstring(l: *mut lua_State, s: *const c_char) {
  unsafe {
    if s.is_null() {
      lua_pushnil(l);
    } else {
      let mut len: usize = 0;
      while *s.add(len) != 0 {
        len += 1;
      }
      lua_pushlstring(l, s, len);
    }
  }
}
