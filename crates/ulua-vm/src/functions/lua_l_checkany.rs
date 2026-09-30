//! Source: `VM/src/laux.cpp:159-163` (hand-ported)

use crate::{
  enums::lua_type::LuaType, functions::lua_type::lua_type, macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`narg` 为合法（伪）索引，`lua_type(l,narg)` 读该栈槽类型；
/// 若为 NONE（缺参）则经 `luaL_error` 抛 "missing argument"（可分配、经 unwind 回退）。正常返回时不写栈。
/// cpp VM/src/laux.cpp:170
pub unsafe fn lua_l_checkany(l: *mut LuaState, narg: i32) {
  unsafe {
    if lua_type(l, narg) == LuaType::None as i32 {
      luaL_error!(l, "missing argument #{}", narg);
    }
  }
}
