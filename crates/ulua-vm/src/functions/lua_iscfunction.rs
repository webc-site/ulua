use crate::{
  functions::index_2_addr::index_2_addr, macros::iscfunction::iscfunction,
  records::lua_state::LuaState, type_aliases::stk_id::StkId,
};

/// `lua_iscfunction` 核心（cpp `lapi.cpp:356`）。只读：经硬化的
/// `index_2_addr` 解析索引后以 `iscfunction!` 读槽判型；越界索引返回
/// `LUA_O_NILOBJECT` 哨兵 → `iscfunction!` 判假 → 0，与 cpp 越界正索引行为
/// 逐位一致。不写栈、不分配、不抛错；伪索引读最多物化 `global.pseudotemp`。
pub(crate) fn lua_iscfunction(l: &LuaState, idx: i32) -> i32 {
  // r19-w2：`index_2_addr` 本就收 `&LuaState`，原 `&*l.read_ptr()` 是把同一只读借用
  // 绕成裸指针再解回来，其 `unsafe` 块仅为该解引用而存在，往返消去后本句全 safe。
  let o: StkId = index_2_addr(l, idx);
  // SAFETY:o 指向栈上有效 TValue 或只读哨兵。
  if unsafe { iscfunction!(o) } { 1 } else { 0 }
}
