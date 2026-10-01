use crate::{functions::index_2_addr::index_2_addr, records::lua_state::LuaState};

/// `lua_isuserdata` 核心（cpp `VM/src/lapi.cpp:381`）。`l` 以引用传入（存活由类型
/// 保证）；`idx` 为任意（伪）索引，越界经硬化的 `index_2_addr` 返回只读哨兵槽
/// （tt=LUA_TNIL，判假）。仅按 tt 判 full/light userdata，不解引用 payload。
/// 不抛错/不分配/不触碰 GC。
pub fn lua_isuserdata(l: &LuaState, idx: i32) -> i32 {
  let o = index_2_addr(l, idx);
  // SAFETY:o 为栈上有效 TValue 或只读哨兵槽，仅读 tag。
  unsafe { ((*o).is_userdata() || (*o).is_lightuserdata()) as i32 }
}
