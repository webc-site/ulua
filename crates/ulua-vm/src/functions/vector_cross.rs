use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_cross(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        (*a.offset(1)) * (*b.offset(2)) - (*a.offset(2)) * (*b.offset(1)),
        (*a.offset(2)) * (*b.offset(0)) - (*a.offset(0)) * (*b.offset(2)),
        (*a.offset(0)) * (*b.offset(1)) - (*a.offset(1)) * (*b.offset(0)),
        0.0f32,
      );
    } else {
      lua_pushvector_lua_state_f32_f32_f32(
        l,
        (*a.offset(1)) * (*b.offset(2)) - (*a.offset(2)) * (*b.offset(1)),
        (*a.offset(2)) * (*b.offset(0)) - (*a.offset(0)) * (*b.offset(2)),
        (*a.offset(0)) * (*b.offset(1)) - (*a.offset(1)) * (*b.offset(0)),
      );
    }

    1
  }
}
