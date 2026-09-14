use core::ffi::c_int;

use crate::{
  functions::{lua_isyieldable::lua_isyieldable, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_coyieldable")]
pub(crate) unsafe extern "C-unwind" fn coyieldable(l: *mut lua_State) -> c_int {
  unsafe {
    lua_pushboolean(l, lua_isyieldable(l));
    1
  }
}
