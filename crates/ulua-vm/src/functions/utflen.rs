use core::{ffi::c_int, ptr::null_mut};

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil, u_posrelat::u_posrelat,
    utf_8_decode::utf_8_decode,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_utflen")]
pub(crate) unsafe extern "C-unwind" fn utflen(l: *mut lua_State) -> c_int {
  unsafe {
    let mut n: i32 = 0;
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);

    let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
    let mut posj = u_posrelat(lua_l_optinteger(l, 3, -1), len);

    luaL_argcheck!(
      l,
      1 <= posi && posi <= len as c_int + 1,
      2,
      "initial position out of string"
    );
    posj -= 1;
    luaL_argcheck!(l, posj < len as c_int, 3, "final position out of string");

    let mut posi = posi - 1;

    while posi <= posj {
      let s1 = utf_8_decode(s.offset(posi as isize), null_mut());
      if s1.is_null() {
        lua_pushnil(l);
        lua_pushinteger(l, (posi + 1) as c_int);
        return 2;
      }
      posi = (s1 as isize - s as isize) as c_int;
      n += 1;
    }

    lua_pushinteger(l, n);
    1
  }
}
