use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32, luai_lerpf::luai_lerpf,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_lerp(l: *mut lua_State) -> c_int {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);
    let t = lua_l_checknumber(l, 3) as f32;

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        luai_lerpf(*a.offset(0), *b.offset(0), t),
        luai_lerpf(*a.offset(1), *b.offset(1), t),
        luai_lerpf(*a.offset(2), *b.offset(2), t),
        luai_lerpf(*a.offset(3), *b.offset(3), t),
      );
    } else {
      lua_pushvector_lua_state_f32_f32_f32(
        l,
        luai_lerpf(*a.offset(0), *b.offset(0), t),
        luai_lerpf(*a.offset(1), *b.offset(1), t),
        luai_lerpf(*a.offset(2), *b.offset(2), t),
      );
    }

    1
  }
}
