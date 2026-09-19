use core::{
  ffi::{CStr, c_char},
  ptr::{null, null_mut},
};

use crate::{
  macros::{curr_func::curr_func, getstr::getstr},
  records::t_string::tstring,
  type_aliases::lua_state::lua_State as lua_State_alias,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn currfuncname(l: *mut lua_State_alias) -> *const c_char {
  unsafe {
    let cl = if (*l).ci > (*l).base_ci {
      curr_func!(l)
    } else {
      null_mut()
    };

    let debugname = if !cl.is_null() && (*cl).is_c != 0 {
      (*cl).inner.c.debugname
    } else {
      null()
    };

    if !debugname.is_null() && CStr::from_ptr(debugname).to_bytes() == b"__namecall" {
      if !(*l).namecall.is_null() {
        getstr((*l).namecall as *const tstring)
      } else {
        null()
      }
    } else {
      debugname
    }
  }
}
