use core::ffi::CStr;

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::{
    getstr::getstr, lua_g_runerror::lua_g_runerror, tsvalue::tsvalue, ttisstring::ttisstring,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_missingmembererror(
  l: *mut lua_State,
  p1: *const TValue,
  p2: *const TValue,
) -> ! {
  unsafe {
    if !ttisstring!(p2) {
      let t1 = lua_t_objtypename(l, p1);
      let t2 = lua_t_objtypename(l, p2);
      lua_g_runerror!(
        l,
        "cannot index {} with a {}",
        CStr::from_ptr(t1).to_string_lossy(),
        CStr::from_ptr(t2).to_string_lossy(),
      )
    } else {
      let t1 = lua_t_objtypename(l, p1);
      let key = tsvalue!(p2);
      lua_g_runerror!(
        l,
        "this {} does not have a key named '{}'",
        CStr::from_ptr(t1).to_string_lossy(),
        CStr::from_ptr(getstr(key)).to_string_lossy(),
      )
    }
  }
}

pub use lua_g_missingmembererror as luaG_missingmembererror;
