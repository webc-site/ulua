use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_pushnil::lua_pushnil, lua_pushvalue::lua_pushvalue,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,TABLE)` 要求索引 1 为表否则抛错回退；upvalue 1
/// 须为 next 迭代 C 函数；`lua_pushvalue`/`lua_pushnil` 连压 3 个返回值（迭代器、原表、nil 游标），
/// `(*l).top` 后须留 ≥3 空槽；push 可触发 GC。
/// cpp VM/src/lbaselib.cpp:229
pub unsafe extern "C-unwind" fn lua_b_pairs(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_pushvalue(l, lua_upvalueindex(1));
    lua_pushvalue(l, 1);
    lua_pushnil(l);
    3
  }
}
