use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checktype::lua_l_checktype,
    lua_pushinteger::lua_pushinteger, lua_rawgeti::lua_rawgeti,
  },
  macros::{lua_isnil::lua_isnil, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 2 为可转整数（`lua_l_checkinteger`，作当前下标）、index 1 为
/// table（`luaL_checktype`），不符即抛错；`lua_rawgeti`/`push` 会读写栈，须在受保护帧内调用。
/// cpp `lbaselib.cpp:238`。
pub unsafe fn lua_b_inext(l: *mut LuaState) -> i32 {
  unsafe {
    let mut i = lua_l_checkinteger(l, 2);
    lua_l_checktype(l, 1, LuaType::Table as i32);
    i += 1; // next value
    lua_pushinteger(l, i);
    lua_rawgeti(l, 1, i);

    if lua_isnil!(l, -1) { 0 } else { 2 }
  }
}

lua_lib_fn!(pub fn lua_b_inext, lua_b_inext_arm);
