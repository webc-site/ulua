//! Source: `VM/src/ltable.cpp:300-304` (单点宏, luaH_clear / luaH_clone /
//! luaH_getn 共用; Rust 侧收敛为一个 pub(crate) 函数)

use crate::records::lua_table::LuaTable;

/// # Safety
/// `t` 必须指向存活的 `LuaTable`。
#[inline]
pub(crate) unsafe fn maybesetaboundary(t: *mut LuaTable, boundary: i32) {
  // Safety: 契约保证 `L` 指向存活 global_State 且其 userdata/registry 边界字段可写，比较赋值仅在阈值跨越时发生
  unsafe {
    if (*t).union.aboundary <= 0 {
      (*t).union.aboundary = -boundary;
    }
  }
}
