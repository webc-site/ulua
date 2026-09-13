//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:306:luaG_methoderror`
//! Source: `VM/src/ldebug.cpp:306-311` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  type_aliases::{lua_state::lua_State, t_string::tstring, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_methoderror(l: *mut lua_State, p1: *const TValue, p2: *const tstring) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);

    lua_g_runerror!(
      l,
      "attempt to call missing method '{}' of {}",
      CStr::from_ptr(getstr(p2)).to_string_lossy(),
      CStr::from_ptr(t1).to_string_lossy()
    )
  }
}

pub use lua_g_methoderror as luaG_methoderror;
