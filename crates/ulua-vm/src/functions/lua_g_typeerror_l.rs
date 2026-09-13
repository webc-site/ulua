//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:242:luaG_typeerrorL`
//! Source: `VM/src/ldebug.cpp:242-247` (hand-ported)

use core::ffi::{CStr, c_char};

use crate::{
  functions::lua_t_objtypename::lua_t_objtypename,
  macros::lua_g_runerror::lua_g_runerror,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_typeerror_l(l: *mut lua_State, o: *const TValue, op: *const c_char) -> ! {
  unsafe {
    let t: *const c_char = lua_t_objtypename(l, o);

    lua_g_runerror!(
      l,
      "attempt to {} a {} value",
      CStr::from_ptr(op).to_string_lossy(),
      CStr::from_ptr(t).to_string_lossy()
    )
  }
}

pub use lua_g_typeerror_l as luaG_typeerrorL;
