use core::{
  ffi::{CStr, c_int},
  ptr::{null, null_mut},
};

use crate::{
  functions::{
    getthread::getthread, lua_l_optinteger::lua_l_optinteger, lua_l_optlstring::lua_l_optlstring,
    lua_l_traceback::lua_l_traceback,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn db_traceback(l: *mut lua_State) -> c_int {
  unsafe {
    let mut arg: i32 = 0;
    let l1 = getthread(l, &mut arg);

    // luaL_optstring(l, arg + 1, NULL)
    // The macro LUA_L_OPTSTRING is a placeholder for the logic:
    // (luaL_optlstring(l, (n), (d), NULL))
    // We call the underlying function directly as per the foundation rules.
    let msg_ptr = lua_l_optlstring(l, arg + 1, null(), null_mut());
    let msg = if msg_ptr.is_null() {
      None
    } else {
      Some(CStr::from_ptr(msg_ptr).to_str().unwrap_or(""))
    };

    let default_level = if l == l1 { 1 } else { 0 };
    let level = lua_l_optinteger(l, arg + 2, default_level);

    luaL_argcheck!(l, level >= 0, arg + 2, "level can't be negative");

    lua_l_traceback(l, l1, msg, level);

    1
  }
}
