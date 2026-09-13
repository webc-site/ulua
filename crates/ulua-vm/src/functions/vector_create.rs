use core::ffi::c_int;

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checknumber::lua_l_checknumber,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_create(l: *mut lua_State) -> c_int {
  unsafe {
    let count = lua_gettop(l);

    let x = lua_l_checknumber(l, 1);
    let y = lua_l_checknumber(l, 2);
    let z = if count >= 3 {
      lua_l_checknumber(l, 3)
    } else {
      0.0
    };

    if LUA_VECTOR_SIZE == 4 {
      let w = if count >= 4 {
        lua_l_checknumber(l, 4)
      } else {
        0.0
      };
      lua_pushvector_lua_state_f32_f32_f32_f32(l, x as f32, y as f32, z as f32, w as f32);
    } else {
      lua_pushvector_lua_state_f32_f32_f32(l, x as f32, y as f32, z as f32);
    }

    1
  }
}
