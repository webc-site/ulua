use alloc::ffi::CString;
use core::ffi::{c_int, c_void};

use crate::{
  functions::{
    lua_getfield::lua_getfield, lua_getmetatable::lua_getmetatable,
    lua_l_typeerror_l::lua_l_typeerror_l, lua_rawequal::lua_rawequal,
    lua_touserdata::lua_touserdata,
  },
  macros::{lua_pop::lua_pop, lua_registryindex::LUA_REGISTRYINDEX},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_l_checkudata")]
pub unsafe fn lua_l_checkudata(l: *mut lua_State, ud: c_int, tname: &str) -> *mut c_void {
  unsafe {
    let p = lua_touserdata(l, ud);
    if !p.is_null() && lua_getmetatable(l, ud) != 0 {
      let c_tname = CString::new(tname).unwrap();
      lua_getfield(l, LUA_REGISTRYINDEX, c_tname.as_ptr());

      if lua_rawequal(l, -1, -2) != 0 {
        lua_pop(l, 2);
        return p;
      }

      lua_pop(l, 2); // remove both metatables if they didn't match
    }

    // lua_l_typeerror_l is l_noret (returns !), so this call never returns.
    lua_l_typeerror_l(l, ud, tname);
  }
}
