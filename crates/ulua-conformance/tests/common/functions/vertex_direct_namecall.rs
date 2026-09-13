use core::{
  ffi::{CStr, c_int, c_void},
  ptr::null_mut,
};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_namecallatom::lua_namecallatom},
  records::lua_state::lua_State,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{lua_vertex_clone::lua_vertex_clone, update_direct_slot::update_direct_slot},
  records::vertex::Vertex,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vertex_direct_namecall(
  l: *mut lua_State,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) -> c_int {
  unsafe {
    let self_ptr = data as *mut Vertex;

    if *cachedslot == 0 {
      update_direct_slot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
      Some(DirectSlot::Clone) => lua_vertex_clone(l, self_ptr),
      _ => {
        let method = lua_namecallatom(l, null_mut());
        let method = if method.is_null() {
          ""
        } else {
          CStr::from_ptr(method).to_str().unwrap_or("")
        };
        lua_l_error_l(
          l,
          c"%s is not a valid method of vertex".as_ptr(),
          format_args!("{method} is not a valid method of vertex"),
        );
        0
      }
    }
  }
}
