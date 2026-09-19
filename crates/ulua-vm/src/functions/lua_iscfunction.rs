use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr, macros::iscfunction::iscfunction,
  records::lua_state::lua_State, type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_iscfunction(l: *mut lua_State, idx: c_int) -> c_int {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: StkId = unsafe { index2addr(l, idx) };
  // SAFETY：o 指向栈上有效 TValue。
  if unsafe { iscfunction!(o) } { 1 } else { 0 }
}
