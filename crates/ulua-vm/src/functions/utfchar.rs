use core::{
  ffi::{c_char, c_int},
  ptr::{null, null_mut},
};

use crate::{
  functions::{
    buffutfchar::buffutfchar, lua_gettop::lua_gettop, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_pushresult::lua_l_pushresult,
    lua_pushlstring::lua_pushlstring,
  },
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

const UTF8BUFFSZ: usize = 8;

#[unsafe(export_name = "ulua_utfchar")]
pub(crate) unsafe extern "C-unwind" fn utfchar(l: *mut lua_State) -> c_int {
  unsafe {
    let mut buff = [0 as c_char; UTF8BUFFSZ];
    let mut charstr = null::<c_char>();

    let n = lua_gettop(l); // number of arguments
    if n == 1 {
      // optimize common case of single char
      let len = buffutfchar(l, 1, buff.as_mut_ptr(), &mut charstr as *mut *const c_char);
      lua_pushlstring(l, charstr, len as usize);
    } else {
      let mut b = LuaLStrbuf {
        p: null_mut(),
        end: null_mut(),
        l: null_mut(),
        storage: null_mut(),
        buffer: [0; LUA_BUFFERSIZE],
      };
      lua_l_buffinit(l, &mut b as *mut LuaLStrbuf);
      let mut i = 1;
      while i <= n {
        let len = buffutfchar(l, i, buff.as_mut_ptr(), &mut charstr as *mut *const c_char);
        lua_l_addlstring(&mut b as *mut LuaLStrbuf, charstr, len as usize);
        i += 1;
      }
      lua_l_pushresult(&mut b as *mut LuaLStrbuf);
    }
    1
  }
}
