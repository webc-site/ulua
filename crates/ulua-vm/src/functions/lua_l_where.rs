//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:71:luaL_where`
//! Source: `VM/src/laux.cpp:71-83` (hand-ported)

use core::ffi::{CStr, c_int};

use crate::{
  functions::{
    currentline::currentline, getluaproto::get_lua_proto, lua_o_chunkid::lua_o_chunkid,
    lua_o_pushfstring::luaO_pushfstring, lua_pushlstring::lua_pushlstring,
    lua_rawcheckstack::lua_rawcheckstack,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_l_where(l: *mut lua_State, level: c_int) {
  unsafe {
    let mut ci = (*l).ci;
    for _ in 0..level {
      if ci == (*l).base_ci {
        lua_rawcheckstack(l, 1);
        lua_pushlstring(l, c"".as_ptr(), 0);
        return;
      }
      ci = ci.sub(1);
    }

    if isLua!(ci) {
      let proto = get_lua_proto(ci);
      let source = (*proto).source;
      let mut chunkbuf = [0; LUA_IDSIZE as usize];
      let chunkid = lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        getstr(source),
        (*source).len as usize,
      );
      let line = currentline(l, ci);
      if line > 0 {
        let chunk = CStr::from_ptr(chunkid).to_string_lossy();
        luaO_pushfstring(l, c"%s:%d: ".as_ptr(), format_args!("{}:{}: ", chunk, line));
        return;
      }
    }

    lua_rawcheckstack(l, 1);
    lua_pushlstring(l, c"".as_ptr(), 0);
  }
}
