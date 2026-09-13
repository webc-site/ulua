use core::{ffi::c_int, mem::zeroed};

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_a_pushvalue::luaA_pushvalue, lua_h_clone::lua_h_clone, lua_l_checktype::lua_l_checktype,
    lua_l_getmetafield::lua_l_getmetafield,
  },
  macros::{hvalue::hvalue, lua_l_argcheck::luaL_argcheck, sethvalue::sethvalue},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[unsafe(export_name = "ulua_tclone")]
pub(crate) unsafe extern "C-unwind" fn tclone(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    luaL_argcheck!(
      l,
      lua_l_getmetafield(l, 1, c"__metatable".as_ptr()) == 0,
      1,
      "table has a protected metatable"
    );

    let tt = lua_h_clone(l, hvalue!((*l).base));

    let mut v: TValue = zeroed();
    sethvalue!(l, &mut v, tt);
    luaA_pushvalue(l, &v);

    1
  }
}
