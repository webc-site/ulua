use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_next::lua_next, lua_pushnil::lua_pushnil,
    lua_settop::lua_settop,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为 table（`luaL_checktype` 校验，不符即抛错）；
/// `lua_next` 读写栈顶并可抛错，须在受保护帧内调用。cpp `lbaselib.cpp:216`。
pub unsafe fn lua_b_next(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_settop(l, 2);

    if lua_next(l, 1) != 0 {
      2
    } else {
      lua_pushnil(l);
      1
    }
  }
}

lua_lib_fn!(pub fn lua_b_next, lua_b_next_arm);
