use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_buffinitsize::lua_l_buffinitsize,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_pushresultsize::lua_l_pushresultsize,
  },
  macros::{lua_l_argcheck::luaL_argcheck, uchar::uchar},
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_str_char")]
pub(crate) unsafe extern "C-unwind" fn str_char(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_gettop(l); // number of arguments

    let mut b = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    let ptr = lua_l_buffinitsize(l, &mut b as *mut LuaLStrbuf, n as usize);

    let mut i = 1;
    while i <= n {
      let c = lua_l_checkinteger(l, i as c_int);
      luaL_argcheck!(
        l,
        i32::from(uchar(c)) == c as c_int,
        i as c_int,
        "invalid value"
      );

      *ptr.offset((i - 1) as isize) = uchar(c) as c_char;
      i += 1;
    }
    lua_l_pushresultsize(&mut b as *mut LuaLStrbuf, n as usize);
    1
  }
}
