use crate::records::lua_table::LuaTable;

/// # Safety
///
/// `t` must point to a valid, properly aligned `LuaTable`.
#[inline(always)]
pub(crate) unsafe fn invalidate_tmcache(t: *mut LuaTable) {
  // Safety: 契约保证 `t` 指向存活 `LuaTable`，此处仅将其 `tmcache` 字段清零
  unsafe {
    (*t).tmcache = 0;
  }
}
