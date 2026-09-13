use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::lua_g_readonlyerror, lua_h_clear::lua_h_clear,
    lua_l_checktype::lua_l_checktype,
  },
  macros::hvalue::hvalue,
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tclear")]
pub(crate) unsafe extern "C-unwind" fn tclear(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);

    let tt = hvalue!((*l).base);

    if (*tt).readonly != 0 {
      lua_g_readonlyerror(l);
    }

    lua_h_clear(tt);
    0
  }
}
