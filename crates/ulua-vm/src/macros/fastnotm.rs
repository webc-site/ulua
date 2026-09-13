use crate::records::lua_table::LuaTable;

/// # Safety
///
/// If `et` is non-null, it must point to a valid, properly aligned `LuaTable`.
#[inline(always)]
pub unsafe fn fastnotm(et: *mut LuaTable, e: i32) -> bool {
  unsafe { et.is_null() || ((*et).tmcache as i32 & (1 << e)) != 0 }
}
