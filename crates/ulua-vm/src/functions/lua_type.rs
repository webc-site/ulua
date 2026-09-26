use core::ptr::eq;

use crate::{
  enums::lua_type::LUA_TNONE,
  functions::index_2_addr::index_2_addr,
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, ttype::ttype},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_type(l: *mut LuaState, idx: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: StkId = unsafe { index_2_addr(l, idx) };

  if eq(o, LUA_O_NILOBJECT) {
    LUA_TNONE
  } else {
    // Safety:o 已指向栈上有效 TValue（非 nilobject 哨兵）。
    unsafe { ttype!(o) as i32 }
  }
}
