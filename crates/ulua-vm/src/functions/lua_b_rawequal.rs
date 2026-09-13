use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_pushboolean::lua_pushboolean, lua_rawequal::lua_rawequal,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_rawequal")]
pub(crate) unsafe extern "C-unwind" fn lua_b_rawequal(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_checkany(l, 2);

    let result = lua_rawequal(l, 1, 2);
    lua_pushboolean(l, result);
    1
  }
}
