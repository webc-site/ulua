use core::ffi::{CStr, c_int, c_void};

use ulua_vm::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_l_error_l::lua_l_error_l},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{lua_vec_2_get::lua_vec_2_get, update_direct_slot::update_direct_slot},
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_newindex(
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
      Some(DirectSlot::Pos) => {
        let pos = lua_l_checkvector(l, 3);
        (*self_ptr).pos[0] = *pos.add(0);
        (*self_ptr).pos[1] = *pos.add(1);
        (*self_ptr).pos[2] = *pos.add(2);
      }
      Some(DirectSlot::Normal) => {
        let normal = lua_l_checkvector(l, 3);
        (*self_ptr).normal[0] = *normal.add(0);
        (*self_ptr).normal[1] = *normal.add(1);
        (*self_ptr).normal[2] = *normal.add(2);
      }
      Some(DirectSlot::UV) => {
        let uv = lua_vec_2_get(l, 3);
        (*self_ptr).uv[0] = (*uv).x;
        (*self_ptr).uv[1] = (*uv).y;
      }
      _ => {
        let name_ptr = luaL_checkstring!(l, 2);
        let name = CStr::from_ptr(name_ptr).to_string_lossy();
        lua_l_error_l(
          l,
          c"%s is not a writable member of vertex".as_ptr(),
          format_args!("{name} is not a writable member of vertex"),
        );
      }
    }
  }
}
