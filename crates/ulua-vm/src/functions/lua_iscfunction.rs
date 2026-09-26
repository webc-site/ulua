use crate::{
  functions::index_2_addr::index_2_addr, macros::iscfunction::iscfunction,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引，使 `index_2_addr` 返回指向栈上有效 TValues 的指针
/// （随后 `iscfunction!` 读该槽判断类型）。cpp `lapi.cpp:356`。
pub(crate) unsafe fn lua_iscfunction(l: *mut LuaState, idx: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: StkId = unsafe { index_2_addr(l, idx) };
  // Safety:o 指向栈上有效 TValue。
  if unsafe { iscfunction!(o) } { 1 } else { 0 }
}
