use crate::{
  functions::{lua_insert::lua_insert, lua_pushboolean::lua_pushboolean},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`r>=0` 时栈顶须有 `r` 个协程 resume 返回值，供
/// `lua_pushboolean`+`lua_insert(-(r+1))` 把状态标志插到返回值之前（要求 `top-base>=r`）；
/// `r<0` 时 -1 须为已移入的错误值。仅重排栈、不分配。cpp `lcorolib.cpp:200`。
pub unsafe fn coresumefinish(l: *mut LuaState, r: i32) -> i32 {
  unsafe {
    if r < 0 {
      lua_pushboolean(l, 0);
      lua_insert(l, -2);
      2
    } else {
      lua_pushboolean(l, 1);
      lua_insert(l, -(r + 1));
      r + 1
    }
  }
}
