use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_pushinteger::lua_pushinteger,
    lua_pushvalue::lua_pushvalue,
  },
  macros::lua_upvalueindex::lua_upvalueindex,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,TABLE)` 要求索引 1 为表否则抛错回退；upvalue 1
/// 须为 ipairs 的迭代 C 函数；`lua_pushvalue`/`lua_pushinteger` 连压 3 个返回值（迭代器、原表、下标 0），
/// `(*l).top` 后须留 ≥3 空槽；push 可触发 GC。
/// cpp VM/src/lbaselib.cpp:248
pub unsafe extern "C-unwind" fn lua_b_ipairs(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    lua_pushvalue(l, lua_upvalueindex(1));
    lua_pushvalue(l, 1);
    lua_pushinteger(l, 0);
    3
  }
}
