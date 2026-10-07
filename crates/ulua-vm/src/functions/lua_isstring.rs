use crate::{
  enums::lua_type::LuaType, functions::lua_type::lua_type, records::lua_state::LuaState,
};

/// `lua_isstring` 核心（cpp `VM/src/lapi.cpp:399`）。`l` 以引用传入（存活由类型
/// 保证）；`idx` 为任意（伪）索引，越界落 `LUA_TNONE` 臂返回 0。仅读槽 tag，
/// 不写栈、不分配、不抛错。
pub fn lua_isstring(l: &LuaState, idx: i32) -> i32 {
  let t = lua_type(l, idx);
  if t == LuaType::String as i32 || t == LuaType::Number as i32 {
    1
  } else {
    0
  }
}
