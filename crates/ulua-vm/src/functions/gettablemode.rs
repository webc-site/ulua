//! Source: `VM/src/lgc.cpp:311-319` (hand-ported)

use core::{ffi::c_char, ptr::null};

use crate::{
  enums::tms::TMS,
  macros::{gfasttm::gfasttm, svalue::svalue},
  records::{global_state::global_State, lua_table::LuaTable},
};

/// # Safety
/// `g` 须为存活 `global_State`（`gfasttm` 读其 fasttm 缓存），`h` 须为存活 `LuaTable`：其 `(*h).metatable`
/// 允许 NULL（NULL 时 gfasttm 直接返回空）；返回的 `const char*` 仅在 `mode` 为字符串时指向该 TValue 内嵌串（生命周期随表），
/// 否则返回 NULL。只读，不分配、不抛错、不回收。
/// cpp VM/src/lgc.cpp:333
pub(crate) unsafe fn gettablemode(g: *mut global_State, h: *mut LuaTable) -> *const c_char {
  unsafe {
    let mode = gfasttm(g, (*h).metatable, TMS::TmMode);
    if !mode.is_null() && (*mode).is_string() {
      return svalue!(mode);
    }
    null()
  }
}
