use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_error_l::lua_l_error_l,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_div")]
pub(crate) unsafe fn int64_div(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let b = lua_l_checkinteger_64(l, 2);

    if b == 0 {
      lua_l_error_l(
        l,
        c"division by zero".as_ptr(),
        core::format_args!("division by zero"),
      );
    }
    if a == i64::MIN && b == -1 {
      lua_l_error_l(
        l,
        c"integer overflow".as_ptr(),
        core::format_args!("integer overflow"),
      );
    }

    let result = a / b;
    lua_pushinteger_64(l, result);

    1
  }
}
