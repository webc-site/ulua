use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32, luaui_signf::luaui_signf,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn vector_sign(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkvector(l, 1);

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_lua_state_f32_f32_f32_f32(
        l,
        luaui_signf(*v.offset(0)),
        luaui_signf(*v.offset(1)),
        luaui_signf(*v.offset(2)),
        luaui_signf(*v.offset(3)),
      );
    } else {
      lua_pushvector_lua_state_f32_f32_f32(
        l,
        luaui_signf(*v.offset(0)),
        luaui_signf(*v.offset(1)),
        luaui_signf(*v.offset(2)),
      );
    }

    1
  }
}
