use core::ffi::{CStr, c_int, c_void};

use crate::{
  functions::{
    lua_getuserdataname::lua_getuserdataname, lua_l_typeerror_l::lua_l_typeerror_l,
    lua_touserdatatagged::lua_touserdatatagged,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_l_checkudatatagged(
  l: *mut lua_State,
  ud: c_int,
  tag: c_int,
) -> *mut c_void {
  unsafe {
    let p = lua_touserdatatagged(l, ud, tag);
    if !p.is_null() {
      return p;
    }

    let tname = lua_getuserdataname(l, tag);
    let tname_str = CStr::from_ptr(tname).to_str().unwrap_or("userdata");
    lua_l_typeerror_l(l, ud, tname_str);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_luaL_checkudatatagged")]
pub unsafe extern "C-unwind" fn lua_l_checkudatatagged_export(
  l: *mut lua_State,
  ud: c_int,
  tag: c_int,
) -> *mut c_void {
  unsafe { lua_l_checkudatatagged(l, ud, tag) }
}
