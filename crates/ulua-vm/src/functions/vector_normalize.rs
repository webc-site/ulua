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

pub(crate) unsafe extern "C-unwind" fn vector_normalize(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);

    if LUA_VECTOR_SIZE == 4 {
      let v0 = *v.offset(0);
      let v1 = *v.offset(1);
      let v2 = *v.offset(2);
      let v3 = *v.offset(3);

      let inv_sqrt = 1.0f32 / (v0 * v0 + v1 * v1 + v2 * v2 + v3 * v3).sqrt();
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        v0 * inv_sqrt,
        v1 * inv_sqrt,
        v2 * inv_sqrt,
        v3 * inv_sqrt,
      );
    } else {
      let v0 = *v.offset(0);
      let v1 = *v.offset(1);
      let v2 = *v.offset(2);

      let inv_sqrt = 1.0f32 / (v0 * v0 + v1 * v1 + v2 * v2).sqrt();
      lua_pushvector_lua_state_f32_f32_f32(l, v0 * inv_sqrt, v1 * inv_sqrt, v2 * inv_sqrt);
    }

    1
  }
}
