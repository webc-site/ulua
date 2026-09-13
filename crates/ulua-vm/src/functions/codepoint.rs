use core::{ffi::c_int, format_args};

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkstack::lua_l_checkstack,
    lua_l_error_l::lua_l_error_l, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, u_posrelat::u_posrelat, utf_8_decode::utf_8_decode,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_codepoint")]
pub(crate) unsafe extern "C-unwind" fn codepoint(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let mut s = lua_l_checklstring(l, 1, &mut len);

    let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
    let pose = u_posrelat(lua_l_optinteger(l, 3, posi), len);

    luaL_argcheck!(l, posi >= 1, 2, "out of range");
    luaL_argcheck!(l, pose <= len as i32, 3, "out of range");

    if posi > pose {
      return 0; // empty interval; return no values
    }

    if (pose as i64 - posi as i64) >= c_int::MAX as i64 {
      lua_l_error_l(
        l,
        c"string slice too long".as_ptr(),
        format_args!("string slice too long"),
      );
    }

    let n = (pose - posi) + 1;
    lua_l_checkstack(l, n, "string slice too long");

    let mut n = 0;
    let se = s.add(pose as usize);
    s = s.add((posi - 1) as usize);

    while s < se {
      let mut code: i32 = 0;
      s = utf_8_decode(s, &mut code);
      if s.is_null() {
        lua_l_error_l(
          l,
          c"invalid UTF-8 code".as_ptr(),
          format_args!("invalid UTF-8 code"),
        );
      }
      lua_pushinteger(l, code);
      n += 1;
    }

    n
  }
}
