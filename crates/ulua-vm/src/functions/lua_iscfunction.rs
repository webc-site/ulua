use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr, macros::iscfunction::iscfunction,
  records::lua_state::lua_State, type_aliases::stk_id::StkId,
};

pub(crate) unsafe fn lua_iscfunction(l: *mut lua_State, idx: c_int) -> c_int {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: StkId = unsafe { index2addr(l, idx) };
  // SAFETY：o 指向栈上有效 TValue。
  if unsafe { iscfunction!(o) } { 1 } else { 0 }
}
