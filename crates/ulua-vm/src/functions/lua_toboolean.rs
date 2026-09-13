use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::l_isfalse::l_isfalse,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_toboolean(l: *mut lua_State, idx: c_int) -> c_int {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: *const TValue = unsafe { index2addr(l, idx) };
  // SAFETY：o 指向栈上有效 TValue。
  (!unsafe { l_isfalse!(o) }) as c_int
}
