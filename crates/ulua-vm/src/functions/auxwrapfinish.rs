//! Source: `VM/src/lcorolib.cpp:144-157` (hand-ported)

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_insert::lua_insert,
    lua_isstring::lua_isstring, lua_l_where::lua_l_where,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈顶（索引 -1）已压入协程 wrap 的返回值或错误对象；
/// `r<0` 表示出错，此时须保证 -1 为可拼接值且 `lua_error` 会抛，须在受保护帧内调用，
/// `r>=0` 时仅透传不触碰栈。cpp `lcorolib.cpp:275`。
pub(crate) unsafe fn auxwrapfinish(l: *mut LuaState, r: i32) -> i32 {
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
