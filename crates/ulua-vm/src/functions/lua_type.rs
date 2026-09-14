use core::{ffi::c_int, ptr::eq};

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::index_2_addr::index2addr,
  macros::{lua_o_nilobject::luaO_nilobject, ttype::ttype},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_type")]
pub unsafe fn lua_type(l: *mut lua_State, idx: c_int) -> c_int {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: StkId = unsafe { index2addr(l, idx) };

  if eq(o, luaO_nilobject) {
    LUA_TNONE
  } else {
    // SAFETY：o 已指向栈上有效 TValue（非 nilobject 哨兵）。
    unsafe { ttype!(o) as c_int }
  }
}
