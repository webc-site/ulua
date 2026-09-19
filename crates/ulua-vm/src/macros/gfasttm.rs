use core::{mem::transmute, ptr::null};

use crate::{
  enums::tms::TMS,
  functions::lua_t_gettm::lua_t_gettm,
  records::{global_state::global_State, lua_t_value::TValue, lua_table::LuaTable},
};

/// # Safety
///
/// If `g` is accessed, it must point to a valid `global_State`.
/// If `et` is non-null, it must point to a valid `LuaTable`.
#[inline(always)]
pub(crate) unsafe fn gfasttm(g: *mut global_State, et: *mut LuaTable, e: i32) -> *const TValue {
  unsafe {
    if et.is_null() {
      null()
    } else {
      let tmcache = (*et).tmcache;
      // The C++ macro: ((et)->tmcache & (1u << (e))) ? NULL : luaT_gettm(...)
      // This means if the bit IS set, we return NULL (it's a "fast" check for absence).
      if (tmcache as u32 & (1u32 << e)) != 0 {
        null()
      } else {
        lua_t_gettm(et, transmute::<i32, TMS>(e), (*g).tmname[e as usize])
      }
    }
  }
}
