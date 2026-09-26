use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_typename::lua_l_typename, lua_pushstring::lua_pushstring,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位为任意值（`lua_l_checkany` 校验非无值，`lua_l_typename` 读其类型名），
/// `lua_pushstring` 写回名称串并可分配/GC。cpp/VM/src/lbaselib.cpp:208 luaB_typeof。
pub unsafe extern "C-unwind" fn lua_b_typeof(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    let name = lua_l_typename(l, 1);
    lua_pushstring(l, name);
    1
  }
}
