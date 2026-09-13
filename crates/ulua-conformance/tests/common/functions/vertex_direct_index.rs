use core::{
  ffi::{CStr, c_int, c_void},
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
  enums::direct_slot::DirectSlot,
  functions::{lua_vec_2_push::lua_vec_2_push, update_direct_slot::update_direct_slot},
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_index(
  l: *mut lua_State,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) {
  unsafe {
    let self_ptr = data as *mut Vertex;

    if *cachedslot == 0 {
      update_direct_slot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
      Some(DirectSlot::Pos) => lua_pushvector_lua_state_f32_f32_f32(
        l,
        (*self_ptr).pos[0],
        (*self_ptr).pos[1],
        (*self_ptr).pos[2],
      ),
      Some(DirectSlot::Normal) => lua_pushvector_lua_state_f32_f32_f32(
        l,
        (*self_ptr).normal[0],
        (*self_ptr).normal[1],
        (*self_ptr).normal[2],
      ),
      Some(DirectSlot::UV) => {
        let uv = lua_vec_2_push(l);
        (*uv).x = (*self_ptr).uv[0];
        (*uv).y = (*self_ptr).uv[1];
      }
      Some(DirectSlot::Sizeof) => lua_pushnumber(l, size_of::<Vertex>() as f64),
      _ => {
        let name_ptr = luaL_checkstring!(l, 2);
        let name = CStr::from_ptr(name_ptr).to_string_lossy();
        lua_l_error_l(
          l,
          c"%s is not a valid member of vertex".as_ptr(),
          format_args!("{name} is not a valid member of vertex"),
        );
      }
    }
  }
}
