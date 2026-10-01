use crate::{macros::lua_lib_fn::lua_lib_fn, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkany(l,1)`、`(l,2)` 要求索引 1、2 均有值否则抛错回退；
/// `lua_rawequal(l,1,2)` 读取这两个栈槽（不触发 __eq 元方法）；`lua_pushboolean` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lbaselib.cpp:158
pub unsafe fn lua_b_rawequal(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_any(1);
    (*l).check_any(2);

    let result = (*l).raw_equal(1, 2) as i32;
    (*l).push_boolean(result != 0);
    1
  }
}

lua_lib_fn!(pub fn lua_b_rawequal, lua_b_rawequal_arm);
