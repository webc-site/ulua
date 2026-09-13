use core::{
  ffi::c_int,
  ptr::{null, null_mut},
};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    addfield::addfield, lua_l_addlstring::lua_l_addlstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger,
    lua_l_optlstring::lua_l_optlstring, lua_l_pushresult::lua_l_pushresult, lua_objlen::lua_objlen,
  },
  macros::hvalue::hvalue,
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tconcat")]
pub(crate) unsafe extern "C-unwind" fn tconcat(l: *mut lua_State) -> c_int {
  unsafe {
    let mut lsep: usize = 0;
    let sep = lua_l_optlstring(l, 2, null(), &mut lsep);
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    let i = lua_l_optinteger(l, 3, 1);
    let last = lua_objlen(l, 1);
    let last = lua_l_optinteger(l, 4, last);

    let t = hvalue!((*l).base);

    let mut b: LuaLStrbuf = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    lua_l_buffinit(l, &mut b);
    let mut current_i = i;
    while current_i < last {
      addfield(l, &mut b, current_i, t);
      if lsep != 0 {
        lua_l_addlstring(&mut b, sep, lsep);
      }
      current_i += 1;
    }
    if current_i == last {
      addfield(l, &mut b, current_i, t);
    }
    lua_l_pushresult(&mut b);
    1
  }
}
