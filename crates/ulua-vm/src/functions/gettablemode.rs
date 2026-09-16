//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:311:gettablemode`
//! Source: `VM/src/lgc.cpp:311-319` (hand-ported)

use core::{ffi::c_char, ptr::null};

use crate::{
  enums::tms::TMS,
  macros::{gfasttm::gfasttm, svalue::svalue, ttisstring::ttisstring},
  records::{global_state::global_State, lua_table::LuaTable},
};

pub(crate) unsafe fn gettablemode(g: *mut global_State, h: *mut LuaTable) -> *const c_char {
  unsafe {
    let mode = gfasttm(g, (*h).metatable, TMS::TmMode as i32);
    if !mode.is_null() && ttisstring!(mode) {
      return svalue!(mode);
    }
    null()
  }
}
