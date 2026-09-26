use crate::{enums::tms::TMS, records::lua_table::LuaTable};

/// cpp `ltm.h:46` `#define fastnotm(et, e) ((et) == NULL || ((et)->tmcache & (1u << (e))))` 对应。
///
/// # Safety
///
/// `et` 非空时必须指向有效、正确对齐的 `LuaTable`。
#[inline(always)]
pub unsafe fn fastnotm(et: *mut LuaTable, e: TMS) -> bool {
  // Safety: 契约保证 `et` 为空或指向存活 `LuaTable`，此处仅读 `tmcache` 位图字段
  unsafe { et.is_null() || ((*et).tmcache as u32 & (1u32 << (e as u32))) != 0 }
}
