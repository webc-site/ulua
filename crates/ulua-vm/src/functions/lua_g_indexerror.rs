//! Source: `VM/src/ldebug.cpp:286-296` (hand-ported)

use core::{ffi::c_char, ptr::null};

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_indexerror(l: *mut LuaState, p1: *const TValue, p2: *const TValue) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);
    let t2: *const c_char = lua_t_objtypename(l, p2);
    let key: *const tstring = if (*p2).is_string() {
      (*p2).as_string_ptr()
    } else {
      null()
    };

    // limit length to make sure we don't generate very long error messages for very long keys
    if !key.is_null() && (*key).len <= 64 {
      lua_g_runerror!(
        l,
        "attempt to index {} with '{}'",
        cstr_cow(t1),
        cstr_cow(getstr(key))
      )
    } else {
      lua_g_runerror!(l, "attempt to index {} with {}", cstr_cow(t1), cstr_cow(t2))
    }
  }
}
