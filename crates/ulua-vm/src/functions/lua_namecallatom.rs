use core::{ffi::c_char, ptr::null};

use crate::{
  macros::{getstr::getstr, lua_s_updateatom::luaS_updateatom},
  type_aliases::lua_state::lua_State as lua_State_alias,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_namecallatom(l: *mut lua_State_alias, atom: *mut i32) -> *const c_char {
  unsafe {
    let s = (*l).namecall;
    if s.is_null() {
      return null();
    }
    if !atom.is_null() {
      luaS_updateatom!(l, s);
      *atom = (*s).atom as i32;
    }
    getstr(s)
  }
}
