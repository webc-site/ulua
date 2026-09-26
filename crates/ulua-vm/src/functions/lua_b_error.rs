//! Source: `VM/src/lbaselib.cpp:48-59` (hand-ported)

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_isstring::lua_isstring,
    lua_l_optinteger::lua_l_optinteger, lua_l_where::lua_l_where, lua_pushvalue::lua_pushvalue,
    lua_settop::lua_settop,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于 error 的受保护帧：栈 1 号位为待抛消息（`lua_settop`/`lua_isstring`/`lua_concat`
/// 读写该槽并可 GC），2 号位可选 level 经 `lua_l_optinteger` 取；末尾 `lua_error` 抛错并 unwind，不返回。
/// cpp/VM/src/lbaselib.cpp:75 luaB_error。
pub(crate) unsafe fn lua_b_error(l: *mut LuaState) -> i32 {
  unsafe {
    let level = lua_l_optinteger(l, 2, 1);
    lua_settop(l, 1);
    if lua_isstring(l, 1) != 0 && level > 0 {
      lua_l_where(l, level);
      lua_pushvalue(l, 1);
      lua_concat(l, 2);
    }
    lua_error(l);
  }
}

lua_lib_fn!(pub(crate) fn lua_b_error, lua_b_error_arm);
