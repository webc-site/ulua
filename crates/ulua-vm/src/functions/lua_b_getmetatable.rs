use crate::{
  functions::{
    lua_getmetatable::lua_getmetatable, lua_l_checkany::lua_l_checkany,
    lua_l_getmetafield::lua_l_getmetafield, lua_pushnil::lua_pushnil,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位为任意值（`lua_l_checkany` 校验非无值，`lua_getmetatable`/
/// `lua_l_getmetafield` 读取并可分配/GC），结果 push 到栈顶。cpp/VM/src/lbaselib.cpp:88 luaB_getmetatable。
pub unsafe fn lua_b_getmetatable(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);

    if lua_getmetatable(l, 1) == 0 {
      lua_pushnil(l);
      return 1; // no metatable
    }

    lua_l_getmetafield(l, 1, c"__metatable".as_ptr());
    1 // returns either __metatable field (if present) or metatable
  }
}

lua_lib_fn!(pub fn lua_b_getmetatable, lua_b_getmetatable_arm);
