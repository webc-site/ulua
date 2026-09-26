use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_o_rawequal_obj::lua_o_rawequal_obj},
  macros::lua_o_nilobject::LUA_O_NILOBJECT,
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `index1`/`index2` 均为合法（伪）索引，使 `index_2_addr` 返回指向栈上
/// 有效 TValue 的指针或 `luaO_nilobject`（伪索引越界时）——后者按不等处理，不解引用。cpp `lapi.cpp:387`。
pub unsafe fn lua_rawequal(l: *mut LuaState, index1: i32, index2: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且索引合法。
  let o1: StkId = unsafe { index_2_addr(l, index1) };
  let o2: StkId = unsafe { index_2_addr(l, index2) };

  if eq(o1, LUA_O_NILOBJECT) || eq(o2, LUA_O_NILOBJECT) {
    0
  } else {
    // Safety:两个指针均指向栈上有效 TValue。
    unsafe { lua_o_rawequal_obj(o1 as *const TValue, o2 as *const TValue) }
  }
}
