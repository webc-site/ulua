//! Source: `VM/src/lbaselib.cpp:75-85` (hand-ported)

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_l_getmetafield::lua_l_getmetafield,
    lua_setmetatable::lua_setmetatable, lua_settop::lua_settop, lua_type::lua_type,
  },
  macros::{lua_l_argexpected::luaL_argexpected, lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 table（`lua_l_checktype`）、2 号位须为 nil/table
/// （`luaL_argexpected`），`lua_l_getmetafield` 检出 `__metatable` 时经 `lua_l_error_l` 抛错，否则
/// `lua_setmetatable` 写元表并可 GC。cpp/VM/src/lbaselib.cpp:100 luaB_setmetatable。
pub(crate) unsafe fn lua_b_setmetatable(l: *mut LuaState) -> i32 {
  unsafe {
    let t = lua_type(l, 2);
    lua_l_checktype(l, 1, LuaType::Table as i32);
    luaL_argexpected!(
      l,
      t == LuaType::Nil as i32 || t == LuaType::Table as i32,
      2,
      "nil or table"
    );
    if lua_l_getmetafield(l, 1, c"__metatable".as_ptr()) != 0 {
      luaL_error!(l, "cannot change a protected metatable");
    }
    lua_settop(l, 2);
    lua_setmetatable(l, 1);
    1
  }
}

lua_lib_fn!(pub(crate) fn lua_b_setmetatable, lua_b_setmetatable_arm);
