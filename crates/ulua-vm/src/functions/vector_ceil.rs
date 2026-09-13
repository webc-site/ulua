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

pub(crate) unsafe extern "C-unwind" fn vector_ceil(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        (*v.offset(0)).ceil(),
        (*v.offset(1)).ceil(),
        (*v.offset(2)).ceil(),
        (*v.offset(3)).ceil(),
      );
    } else {
      lua_pushvector_lua_state_f32_f32_f32(
        l,
        (*v.offset(0)).ceil(),
        (*v.offset(1)).ceil(),
        (*v.offset(2)).ceil(),
      );
    }

    1
  }
}
