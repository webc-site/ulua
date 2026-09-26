use crate::{
  enums::lua_type::LuaType,
  functions::{lua_objlen::lua_objlen, lua_pushinteger::lua_pushinteger, lua_type::lua_type},
  macros::{lua_l_argcheck::luaL_argcheck, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位须为 table 或 string（`lua_type`+`luaL_argcheck` 校验，否则抛错），
/// `lua_objlen` 读该槽、`lua_pushinteger` 写回结果并可分配/GC。cpp/VM/src/lbaselib.cpp:185 luaB_rawlen。
pub unsafe fn lua_b_rawlen(l: *mut LuaState) -> i32 {
  unsafe {
    let tt = lua_type(l, 1);

    luaL_argcheck!(
      l,
      tt == LuaType::Table as i32 || tt == LuaType::String as i32,
      1,
      "table or string expected"
    );

    let len = lua_objlen(l, 1);
    lua_pushinteger(l, len);

    1
  }
}

lua_lib_fn!(pub fn lua_b_rawlen, lua_b_rawlen_arm);
