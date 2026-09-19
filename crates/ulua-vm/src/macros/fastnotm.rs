use crate::{enums::tms::TMS, records::lua_table::LuaTable};

/// cpp `ltm.h:46` `#define fastnotm(et, e) ((et) == NULL || ((et)->tmcache & (1u << (e))))` 对应。
///
/// # Safety
///
/// `et` 非空时必须指向有效、正确对齐的 `LuaTable`。
#[inline(always)]
pub unsafe fn fastnotm(et: *mut LuaTable, e: TMS) -> bool {
  unsafe { et.is_null() || ((*et).tmcache as u32 & (1u32 << (e as u32))) != 0 }
}
