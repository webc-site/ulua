use crate::{
  functions::index_2_addr::index_2_addr, macros::l_isfalse::l_isfalse,
  records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// `lua_toboolean` 核心（cpp `VM/src/lapi.cpp:474`）。只读：经硬化的
/// `index_2_addr` 解析索引（越界/0/负越界/越界 upvalue 伪索引返回
/// `LUA_O_NILOBJECT` 哨兵，`l_isfalse!` 对哨兵 tt=LUA_TNIL 判真 → 返回 0，
/// 与 cpp 越界正索引行为逐位一致），仅读值判真假，不写栈、不分配、不抛错；
/// 伪索引读最多物化 `global.pseudotemp`（经 `l.global` 指针，不写 `l` 自身）。
pub(crate) fn lua_toboolean(l: &LuaState, idx: i32) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已对任意 idx 硬化（越界返回
  // 哨兵，无栈外指针算术），`read_ptr` 只读转发契约成立（本函数不写 `l`）。
  let o: *const TValue = unsafe { index_2_addr(l.read_ptr(), idx) };
  // SAFETY:o 指向栈上有效 TValue 或只读哨兵。
  (!unsafe { l_isfalse!(o) }) as i32
}
