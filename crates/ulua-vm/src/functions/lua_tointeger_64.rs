use crate::{
  functions::index_2_addr::index_2_addr, macros::lvalue::lvalue, records::lua_state::LuaState,
};

/// cpp `lua_tointeger64`（`VM/src/lapi.cpp:480-490`）的内部精简版。
///
/// cpp 的 `int* isinteger` out 参数在本 crate 的全部调用点（laux.cpp 对应处
/// `luaL_checkinteger64`/`luaL_tolstring`/`luaL_addvalueany`）均传 nullptr，
/// 故改为直接返回 i64：非整数返回 0。只读：越界索引经硬化的
/// `index_2_addr` 返回 `LUA_O_NILOBJECT` 哨兵 → `is_integer` 判假 → 0，
/// 与 cpp 越界正索引行为逐位一致；不抛错/不分配；伪索引读最多物化
/// `global.pseudotemp`。cpp/VM/src/lapi.cpp:480 lua_tointeger64。
pub(crate) fn lua_tointeger_64(l: &LuaState, idx: i32) -> i64 {
  // r19-w2：`index_2_addr` 本就收 `&LuaState`，原 `&*l.read_ptr()` 往返消去后本句全 safe。
  let o = index_2_addr(l, idx);
  // SAFETY:o 指向栈上有效 TValue 或只读哨兵；哨兵 tt=LUA_TNIL 使 is_integer
  // 判假走 else 臂，不解引用 union 整数视图。
  if unsafe { (*o).is_integer() } {
    unsafe { lvalue!(o) }
  } else {
    0
  }
}
