//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:286:luaG_indexerror`
//! Source: `VM/src/ldebug.cpp:286-296` (hand-ported)

use core::{
  ffi::{CStr, c_char},
  ptr::null,
};

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::{
    getstr::getstr, lua_g_runerror::lua_g_runerror, tsvalue::tsvalue, ttisstring::ttisstring,
  },
  type_aliases::{lua_state::lua_State, t_string::tstring, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_indexerror(l: *mut lua_State, p1: *const TValue, p2: *const TValue) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    let key: *const tstring = if ttisstring!(p2) {
      tsvalue!(p2)
    } else {
      null()
    };

    // limit length to make sure we don't generate very long error messages for very long keys
    if !key.is_null() && (*key).len <= 64 {
      lua_g_runerror!(
        l,
        "attempt to index {} with '{}'",
        CStr::from_ptr(t1).to_string_lossy(),
        CStr::from_ptr(getstr(key)).to_string_lossy()
      )
    } else {
      lua_g_runerror!(
        l,
        "attempt to index {} with {}",
        CStr::from_ptr(t1).to_string_lossy(),
        CStr::from_ptr(t2).to_string_lossy()
      )
    }
  }
}

pub use lua_g_indexerror as luaG_indexerror;
