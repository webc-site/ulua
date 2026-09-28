use crate::{
  enums::lua_type::LuaType,
  macros::{lua_lib_fn::lua_lib_fn, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checktype(l,1,TABLE)` 要求索引 1 为表否则抛错回退；upvalue 1
/// 须为 next 迭代 C 函数；`lua_pushvalue`/`lua_pushnil` 连压 3 个返回值（迭代器、原表、nil 游标），
/// `(*l).top` 后须留 ≥3 空槽；push 可触发 GC。
/// cpp VM/src/lbaselib.cpp:229
pub unsafe fn lua_b_pairs(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);
    (*l).push_value(lua_upvalueindex(1));
    (*l).push_value(1);
    (*l).push_nil();
    3
  }
}

lua_lib_fn!(pub fn lua_b_pairs, lua_b_pairs_arm);
