use core::ffi::c_int;

use ulua_vm::{
  functions::{
    lua_l_error_l::lua_l_error_l, lua_pushinteger::lua_pushinteger, lua_setfield::lua_setfield,
    lua_tolightuserdata::lua_tolightuserdata,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn cpcall_test(l: *mut LuaState) -> c_int {
  unsafe {
    let should_fail = *(lua_tolightuserdata(l, 1) as *const bool);

    if should_fail {
      lua_l_error_l(l, c"Failed".as_ptr(), format_args!("Failed"));
    } else {
      lua_pushinteger(l, 123);
      lua_setfield(l, LUA_GLOBALSINDEX, c"cpcallvalue".as_ptr());
    }

    0
  }
}
