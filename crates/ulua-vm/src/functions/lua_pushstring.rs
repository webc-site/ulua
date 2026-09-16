use core::ffi::{CStr, c_char};

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
      // strlen 等价：CStr 零拷贝扫描至 NUL
      let len = CStr::from_ptr(s).to_bytes().len();
      lua_pushlstring(l, s, len);
    }
  }
}
