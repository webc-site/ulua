//! Node: `cxx:Macro:Luau.VM:VM/src/ltable.cpp:300:maybesetaboundary`
//! Source: `VM/src/ltable.cpp:300-304` (单点宏, luaH_clear / luaH_clone /
//! luaH_getn 共用; Rust 侧收敛为一个 pub(crate) 函数)

use crate::records::lua_table::LuaTable;

/// # Safety
/// `t` 必须指向存活的 `LuaTable`。
#[inline]
pub(crate) unsafe fn maybesetaboundary(t: *mut LuaTable, boundary: i32) {
  unsafe {
    if (*t).union.aboundary <= 0 {
      (*t).union.aboundary = -boundary;
    }
  }
}
