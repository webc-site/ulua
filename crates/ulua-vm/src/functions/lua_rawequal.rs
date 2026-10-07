use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_o_rawequal_obj::lua_o_rawequal_obj},
  macros::lua_o_nilobject::LUA_O_NILOBJECT,
  records::lua_state::LuaState,
};

/// `lua_rawequal` 核心（cpp `VM/src/lapi.cpp:387`）。`l` 以引用传入（存活由类型保证）；
/// `index1`/`index2` 为任意（伪）索引，越界经硬化的 `index_2_addr` 返回哨兵槽——
/// 哨兵按不等处理，不解引用。仅读两槽值，不写栈、不分配、不抛错（不触发 `__eq`）。
pub fn lua_rawequal(l: &LuaState, index1: i32, index2: i32) -> i32 {
  let o1 = index_2_addr(l, index1);
  let o2 = index_2_addr(l, index2);

  if eq(o1, LUA_O_NILOBJECT) || eq(o2, LUA_O_NILOBJECT) {
    0
  } else {
    // SAFETY:两个指针均指向栈上有效 TValue，引用重建仅收形（非空/对齐由指针窗内解引用保证）。
    unsafe { lua_o_rawequal_obj(&*o1, &*o2) }
  }
}
