use core::ptr::eq;

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::index_2_addr::index_2_addr,
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, ttype::ttype},
  records::lua_state::LuaState,
};

/// `lua_type` 核心（cpp `VM/src/lapi.cpp:165`）。`l` 以引用传入（存活由类型保证）；
/// `idx` 为任意（伪）索引，越界经硬化的 `index_2_addr` 收敛为 `LUA_TNONE`。
/// 仅读槽 tag，不写栈、不分配、不抛错。
pub fn lua_type(l: &LuaState, idx: i32) -> i32 {
  let o = index_2_addr(l, idx);

  if eq(o, LUA_O_NILOBJECT) {
    LUA_TNONE
  } else {
    // SAFETY:o 已指向栈上有效 TValue（非 nilobject 哨兵），仅读 tag。
    unsafe { ttype!(o) as i32 }
  }
}
