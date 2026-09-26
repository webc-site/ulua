use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_lessthan::lua_v_lessthan},
  macros::lua_o_nilobject::LUA_O_NILOBJECT,
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
///
/// `l` 须为有效存活的 `LuaState`，`index1`/`index2` 须为栈内有效索引。
pub unsafe fn lua_lessthan(l: *mut LuaState, index1: i32, index2: i32) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、索引 1/2 栈槽可读，lua_v_lessthan 与 TM 调用沿用该帧栈界
  unsafe {
    let o1: StkId = index_2_addr(l, index1);
    let o2: StkId = index_2_addr(l, index2);

    let nil_ptr = LUA_O_NILOBJECT;

    if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
      0
    } else {
      lua_v_lessthan(l, o1 as *const TValue, o2 as *const TValue)
    }
  }
}
