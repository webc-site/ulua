use crate::{
  functions::index_2_addr::index_2_addr, macros::l_isfalse::l_isfalse,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `idx` 为合法（伪）索引，使 `index_2_addr(l,idx)` 返回可读 `TValue*`
/// （NONE 索引返回 LUA_O_NILOBJECT 哨兵，`l_isfalse!` 对哨兵判为 false）；仅读值判真假，不写栈、不分配、不抛错。
/// cpp VM/src/lapi.cpp:474
pub unsafe fn lua_toboolean(l: *mut LuaState, idx: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o: *const TValue = unsafe { index_2_addr(l, idx) };
  // Safety:o 指向栈上有效 TValue。
  (!unsafe { l_isfalse!(o) }) as i32
}
