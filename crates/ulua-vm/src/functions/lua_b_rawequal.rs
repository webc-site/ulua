use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_pushboolean::lua_pushboolean, lua_rawequal::lua_rawequal,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkany(l,1)`、`(l,2)` 要求索引 1、2 均有值否则抛错回退；
/// `lua_rawequal(l,1,2)` 读取这两个栈槽（不触发 __eq 元方法）；`lua_pushboolean` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lbaselib.cpp:158
pub unsafe fn lua_b_rawequal(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_checkany(l, 2);

    let result = lua_rawequal(l, 1, 2);
    lua_pushboolean(l, result);
    1
  }
}

lua_lib_fn!(pub fn lua_b_rawequal, lua_b_rawequal_arm);
