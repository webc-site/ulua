//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:249:luaG_forerrorL`
//! Source: `VM/src/ldebug.cpp:249-254` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::lua_g_runerror::lua_g_runerror,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_forerror_l(l: *mut lua_State, o: *const TValue, what: *const c_char) -> ! {
  unsafe {
    let t: *const c_char = lua_t_objtypename(l, o);

    lua_g_runerror!(
      l,
      "invalid 'for' {} (number expected, got {})",
      CStr::from_ptr(what).to_string_lossy(),
      CStr::from_ptr(t).to_string_lossy()
    )
  }
}

pub use lua_g_forerror_l as luaG_forerror_l;
