use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_call::lua_call, lua_l_checktype::lua_l_checktype, lua_next::lua_next,
    lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
  },
  macros::{lua_isnil::lua_isnil, lua_lib_fn::lua_lib_fn, lua_pop::lua_pop},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为 table、index 2 为 function（`luaL_checktype` 校验，
/// 不符即抛错）；循环内 `lua_next`/`lua_call` 会读写栈、可 GC 可抛错，须在受保护帧内调用。
/// cpp `ltablib.cpp:35`。
pub unsafe fn foreach(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_l_checktype(l, 2, LuaType::Function as i32);
    lua_pushnil(l); // first key
    while lua_next(l, 1) != 0 {
      lua_pushvalue(l, 2); // function
      lua_pushvalue(l, -3); // key
      lua_pushvalue(l, -3); // value
      lua_call(l, 2, 1);
      if !lua_isnil!(l, -1) {
        return 1;
      }
      lua_pop(l, 2); // remove value and result
    }
    0
  }
}

lua_lib_fn!(pub fn foreach, foreach_arm);
