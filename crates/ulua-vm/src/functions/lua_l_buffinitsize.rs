use core::ffi::c_char;

use crate::{
  functions::{lua_l_buffinit::lua_l_buffinit, lua_l_prepbuffsize::lua_l_prepbuffsize},
  records::lua_l_strbuf::LuaLStrbuf,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe fn lua_l_buffinitsize(
  l: *mut lua_State,
  b: *mut LuaLStrbuf,
  size: usize,
) -> *mut c_char {
  unsafe {
    lua_l_buffinit(l, b);
    lua_l_prepbuffsize(b, size)
  }
}
