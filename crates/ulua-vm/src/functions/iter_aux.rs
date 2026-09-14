use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_error_l::lua_l_error_l,
    lua_pushinteger::lua_pushinteger, utf_8_decode::utf_8_decode,
  },
  macros::{iscont::iscont, lua_tointeger::lua_tointeger},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_iter_aux")]
pub(crate) unsafe extern "C-unwind" fn iter_aux(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut n = lua_tointeger!(l, 2) - 1;

    if n < 0 {
      n = 0;
    } else if n < len as c_int {
      n += 1;
      while iscont(s.add(n as usize)) {
        n += 1;
      }
    }

    if n >= len as c_int {
      0
    } else {
      let mut code: i32 = 0;
      let next = utf_8_decode(s.add(n as usize), &mut code);
      if next.is_null() || iscont(next) {
        lua_l_error_l(
          l,
          c"invalid UTF-8 code".as_ptr(),
          core::format_args!("invalid UTF-8 code"),
        );
      }
      lua_pushinteger(l, n + 1);
      lua_pushinteger(l, code);
      2
    }
  }
}
