use crate::{
  functions::index_2_addr::index_2_addr, macros::lvalue::lvalue, records::lua_state::LuaState,
};

/// cpp `lua_tointeger64`（`VM/src/lapi.cpp:480-490`）的内部精简版。
///
/// cpp 的 `int* isinteger` out 参数在本 crate 的全部调用点（laux.cpp 对应处
/// `luaL_checkinteger64`/`luaL_tolstring`/`luaL_addvalueany`）均传 nullptr，
/// 故改为直接返回 i64：非整数返回 0。
///
/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为栈内合法 StkId（越界返回可读的 `LUA_O_NILOBJECT`），
/// 命中整数时 `lvalue` 解引用 union 的整数视图。非整数返回 0（本 crate 全部调用点均传 nullptr 作 `isinteger` 出参）。
/// 不抛错/不分配。cpp/VM/src/lapi.cpp:480 lua_tointeger64。
pub(crate) unsafe fn lua_tointeger_64(l: *mut LuaState, idx: i32) -> i64 {
  unsafe {
    let o = index_2_addr(l, idx);
    if (*o).is_integer() { lvalue!(o) } else { 0 }
  }
}
