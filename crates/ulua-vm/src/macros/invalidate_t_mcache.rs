use crate::records::lua_table::LuaTable;

/// # Safety
///
/// `t` must point to a valid, properly aligned `LuaTable`.
#[inline(always)]
pub(crate) unsafe fn invalidate_tmcache(t: *mut LuaTable) {
  unsafe {
    (*t).tmcache = 0;
  }
}
