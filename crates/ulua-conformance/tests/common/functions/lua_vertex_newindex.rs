use core::ffi::{CStr, c_char};

use ulua_vm::{
  functions::{lua_l_checkvector::lua_l_checkvector, lua_l_error_l::lua_l_error_l},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::functions::{lua_vec_2_get::lua_vec_2_get, lua_vertex_get::lua_vertex_get};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_newindex(l: *mut lua_State) -> i32 {
  unsafe {
    let v = lua_vertex_get(l, 1);
    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr as *const c_char)
      .to_str()
      .unwrap_or("");

    if name == "pos" {
      let pos = lua_l_checkvector(l, 3);
      (*v).pos[0] = *pos;
      (*v).pos[1] = *pos.add(1);
      (*v).pos[2] = *pos.add(2);
    } else if name == "normal" {
      let normal = lua_l_checkvector(l, 3);
      (*v).normal[0] = *normal;
      (*v).normal[1] = *normal.add(1);
      (*v).normal[2] = *normal.add(2);
    } else if name == "uv" {
      let uv = lua_vec_2_get(l, 3);
      (*v).uv[0] = (*uv).x;
      (*v).uv[1] = (*uv).y;
    } else {
      lua_l_error_l(
        l,
        c"%s is not a writable member of vertex".as_ptr(),
        core::format_args!("{} is not a writable member of vertex", name),
      );
    }

    0
  }
}
