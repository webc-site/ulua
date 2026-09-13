use core::{ffi::c_int, ptr::null_mut};

use crate::{
  functions::{lua_l_checkany::lua_l_checkany, lua_l_tolstring::lua_l_tolstring},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_tostring")]
pub(crate) unsafe extern "C-unwind" fn lua_b_tostring(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    lua_l_tolstring(l, 1, null_mut());
    1
  }
}
