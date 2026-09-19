//! Source: `VM/src/lcorolib.cpp:144-157` (hand-ported)

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_insert::lua_insert,
    lua_isstring::lua_isstring, lua_l_where::lua_l_where,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn auxwrapfinish(l: *mut lua_State, r: i32) -> i32 {
  unsafe {
    if r < 0 {
      if lua_isstring(l, -1) != 0 {
        lua_l_where(l, 1);
        lua_insert(l, -2);
        lua_concat(l, 2);
      }
      lua_error(l);
    }
    r
  }
}
