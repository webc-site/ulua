use crate::{
  enums::lua_type::LuaType,
  functions::{lua_tointeger_64::lua_tointeger_64, lua_type::lua_type, tag_error::tag_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可抛错的受保护帧；`narg` 为合法栈索引，`lua_type` 读其类型，非 Integer 时经
/// `tag_error` 抛错并 unwind（此时不返回）；通过后 `lua_tointeger_64` 取 64 位整数值。cpp/VM/src/laux.cpp:235 luaL_checkinteger64。
pub unsafe fn lua_l_checkinteger_64(l: *mut LuaState, narg: i32) -> i64 {
  unsafe {
    if lua_type(l, narg) != (LuaType::Integer as i32) {
      tag_error(l, narg, LuaType::Integer as i32);
    }

    lua_tointeger_64(l, narg)
  }
}
