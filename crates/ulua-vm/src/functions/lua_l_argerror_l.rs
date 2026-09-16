use core::ffi::{CStr, c_int};

use crate::{
  functions::{currfuncname::currfuncname, lua_l_error_l::lua_l_error_l},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_l_argerror_l"))]
pub unsafe fn lua_l_argerror_l(l: *mut lua_State, narg: c_int, extramsg: &str) -> ! {
  unsafe {
    let fname = currfuncname(l);

    if !fname.is_null() {
      let fname = CStr::from_ptr(fname).to_string_lossy();
      lua_l_error_l(
        l,
        c"invalid argument #%d to '%s' (%s)".as_ptr(),
        format_args!("invalid argument #{} to '{}' ({})", narg, fname, extramsg),
      );
    } else {
      lua_l_error_l(
        l,
        c"invalid argument #%d (%s)".as_ptr(),
        format_args!("invalid argument #{} ({})", narg, extramsg),
      )
    }
  }
}

pub use lua_l_argerror_l as luaL_argerrorL;
