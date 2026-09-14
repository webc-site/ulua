use core::ffi::{c_char, c_int};

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tolstring::lua_tolstring, tag_error::tag_error},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_checklstring(l: *mut lua_State, narg: c_int, len: *mut usize) -> *const c_char {
  unsafe {
    let s = lua_tolstring(l, narg, len);
    if s.is_null() {
      tag_error(l, narg, LuaType::String as c_int);
    }
    s
  }
}
