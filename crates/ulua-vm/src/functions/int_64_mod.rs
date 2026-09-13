use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_error_l::lua_l_error_l,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

#[unsafe(export_name = "ulua_int64_mod")]
pub(crate) unsafe fn int64_mod(l: *mut LuaState) -> c_int {
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

    let remainder = if (a != i64::MIN) || (b != -1) {
      let r = a % b;
      if r != 0 && ((a < 0) != (b < 0)) {
        r + b
      } else {
        r
      }
    } else {
      0
    };

    lua_pushinteger_64(l, remainder);

    1
  }
}
