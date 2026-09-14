use ulua_vm::{
  functions::{
    lua_l_checkvector::lua_l_checkvector,
    lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::lua_vector_size::LUA_VECTOR_SIZE,
  records::lua_state::lua_State,
};
pub(crate) fn lua_vector_cross(l: *mut lua_State) -> i32 {
  unsafe {
    let a = lua_l_checkvector(l, 1);
    let b = lua_l_checkvector(l, 2);

    let x = (*a.add(1)) * (*b.add(2)) - (*a.add(2)) * (*b.add(1));
    let y = (*a.add(2)) * (*b.add(0)) - (*a.add(0)) * (*b.add(2));
    let z = (*a.add(0)) * (*b.add(1)) - (*a.add(1)) * (*b.add(0));

    if LUA_VECTOR_SIZE == 4 {
      lua_pushvector_4(l, x, y, z, 0.0);
    } else {
      lua_pushvector_3(l, x, y, z);
    }

    1
  }
}

unsafe fn lua_pushvector_4(l: *mut lua_State, x: f32, y: f32, z: f32, w: f32) {
  unsafe {
    lua_pushvector_lua_state_f32_f32_f32_f32(l, x, y, z, w);
  }
}

unsafe fn lua_pushvector_3(l: *mut lua_State, x: f32, y: f32, z: f32) {
  unsafe {
    lua_pushvector_lua_state_f32_f32_f32(l, x, y, z);
  }
}
