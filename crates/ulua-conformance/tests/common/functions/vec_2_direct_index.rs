use core::{
  ffi::{CStr, c_int, c_void},
  mem::size_of,
};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_pushnumber::lua_pushnumber},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::{
  enums::direct_slot::DirectSlot,
  functions::{lua_vec_2_push::lua_vec_2_push, update_direct_slot::update_direct_slot},
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vec_2_direct_index(
  l: *mut lua_State,
  data: *mut c_void,
  atom: c_int,
  cachedslot: *mut u16,
  _utag: c_int,
) {
  unsafe {
    let self_ptr = data as *mut Vec2;

    if *cachedslot == 0 {
      update_direct_slot(atom, cachedslot);
    }

    match DirectSlot::from_u16(*cachedslot) {
      Some(DirectSlot::X) => lua_pushnumber(l, (*self_ptr).x as f64),
      Some(DirectSlot::Y) => lua_pushnumber(l, (*self_ptr).y as f64),
      Some(DirectSlot::Magnitude) => lua_pushnumber(
        l,
        ((*self_ptr).x * (*self_ptr).x + (*self_ptr).y * (*self_ptr).y).sqrt() as f64,
      ),
      Some(DirectSlot::Unit) => {
        let inv = 1.0 / ((*self_ptr).x * (*self_ptr).x + (*self_ptr).y * (*self_ptr).y).sqrt();
        let result = lua_vec_2_push(l);
        (*result).x = (*self_ptr).x * inv;
        (*result).y = (*self_ptr).y * inv;
      }
      Some(DirectSlot::Sizeof) => lua_pushnumber(l, size_of::<Vec2>() as f64),
      _ => {
        let name_ptr = luaL_checkstring!(l, 2);
        let name = CStr::from_ptr(name_ptr).to_string_lossy();
        lua_l_error_l(
          l,
          c"%s is not a valid member of vec2".as_ptr(),
          format_args!("{name} is not a valid member of vec2"),
        );
      }
    }
  }
}
