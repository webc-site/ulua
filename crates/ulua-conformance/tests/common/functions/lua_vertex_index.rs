use core::{
  ffi::{CStr, c_int},
  mem::size_of,
};

use ulua_vm::{
  functions::{
    lua_l_error_l::lua_l_error_l, lua_pushnumber::lua_pushnumber,
    lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32,
  },
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::{
  functions::{lua_vec_2_push::lua_vec_2_push, lua_vertex_get::lua_vertex_get},
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_index(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_vertex_get(l, 1);
    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr).to_str().unwrap_or("");

    if name == "pos" {
      lua_pushvector_lua_state_f32_f32_f32(l, (*v).pos[0], (*v).pos[1], (*v).pos[2]);
      return 1;
    }

    if name == "normal" {
      lua_pushvector_lua_state_f32_f32_f32(l, (*v).normal[0], (*v).normal[1], (*v).normal[2]);
      return 1;
    }

    if name == "uv" {
      let uv = lua_vec_2_push(l);
      (*uv).x = (*v).uv[0];
      (*uv).y = (*v).uv[1];
      return 1;
    }

    if name == "sizeof" {
      lua_pushnumber(l, size_of::<Vertex>() as f64);
      return 1;
    }

    lua_l_error_l(
      l,
      c"%s is not a valid member of vertex".as_ptr(),
      format_args!("{name} is not a valid member of vertex"),
    );
    0
  }
}
