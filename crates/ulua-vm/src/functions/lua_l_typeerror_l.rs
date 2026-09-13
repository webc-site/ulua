use core::{
  ffi::{CStr, c_char, c_int},
  hint::unreachable_unchecked,
};

use crate::{
  functions::{
    currfuncname::currfuncname, lua_a_toobject::luaA_toobject, lua_l_error_l::lua_l_error_l,
    lua_t_objtypename::lua_t_objtypename,
  },
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_typeerror_l(l: *mut lua_State, narg: c_int, tname: &str) -> ! {
  unsafe {
    let fname: *const c_char = currfuncname(l);
    let obj: *const TValue = luaA_toobject(l, narg);

    if !obj.is_null() {
      let objtypename = lua_t_objtypename(l, obj);
      let objtypename = CStr::from_ptr(objtypename).to_string_lossy();

      if !fname.is_null() {
        let fname = CStr::from_ptr(fname).to_string_lossy();
        lua_l_error_l(
          l,
          c"invalid argument #%d to '%s' (%s expected, got %s)".as_ptr(),
          format_args!(
            "invalid argument #{} to '{}' ({} expected, got {})",
            narg, fname, tname, objtypename
          ),
        );
      } else {
        lua_l_error_l(
          l,
          c"invalid argument #%d (%s expected, got %s)".as_ptr(),
          format_args!(
            "invalid argument #{} ({} expected, got {})",
            narg, tname, objtypename
          ),
        );
      }
    } else {
      if !fname.is_null() {
        let fname = CStr::from_ptr(fname).to_string_lossy();
        lua_l_error_l(
          l,
          c"missing argument #%d to '%s' (%s expected)".as_ptr(),
          format_args!(
            "missing argument #{} to '{}' ({} expected)",
            narg, fname, tname
          ),
        );
      } else {
        lua_l_error_l(
          l,
          c"missing argument #%d (%s expected)".as_ptr(),
          format_args!("missing argument #{} ({} expected)", narg, tname),
        );
      }
    }

    unreachable_unchecked()
  }
}
