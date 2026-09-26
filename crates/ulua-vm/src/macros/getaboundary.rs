use crate::records::lua_table::LuaTable;

/// # Safety
///
/// `t` 必须指向存活的 `LuaTable`（读其 `union.aboundary` 与 `sizearray` 字段）。
#[inline(always)]
pub(crate) unsafe fn getaboundary(t: *const LuaTable) -> i32 {
  // Safety: 契约保证 `t` 指向存活 LuaTable，两字段均为普通数组成员
  unsafe {
    if (*t).union.aboundary < 0 {
      -(*t).union.aboundary
    } else {
      (*t).sizearray
    }
  }
}
