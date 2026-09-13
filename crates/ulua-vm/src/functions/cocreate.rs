use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_newthread::lua_newthread, lua_xpush::lua_xpush,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_cocreate")]
pub(crate) unsafe extern "C-unwind" fn cocreate(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Function as c_int);

    let nl = lua_newthread(l);
    lua_xpush(l, nl, 1);

    1
  }
}
