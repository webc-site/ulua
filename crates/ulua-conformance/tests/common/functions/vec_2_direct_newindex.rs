use core::ffi::{CStr, c_int, c_void};

use ulua_vm::{
  functions::{lua_l_checknumber::luaL_checknumber, lua_l_error_l::lua_l_error_l},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::{
  enums::direct_slot::DirectSlot, functions::update_direct_slot::update_direct_slot,
  records::vec_2_conformance_ir_hooks::Vec2,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn vec_2_direct_newindex(
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
      Some(DirectSlot::X) => (*self_ptr).x = luaL_checknumber(l, 3) as f32,
      Some(DirectSlot::Y) => (*self_ptr).y = luaL_checknumber(l, 3) as f32,
      _ => {
        let name_ptr = luaL_checkstring!(l, 2);
        let name = CStr::from_ptr(name_ptr).to_string_lossy();
        lua_l_error_l(
          l,
          c"%s is not a writable member of vec2".as_ptr(),
          format_args!("{name} is not a writable member of vec2"),
        );
      }
    }
  }
}
