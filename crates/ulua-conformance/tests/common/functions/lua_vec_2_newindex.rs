use core::ffi::{CStr, c_char};

use ulua_vm::{
  functions::{lua_l_checknumber::luaL_checknumber, lua_l_error_l::lua_l_error_l},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::functions::lua_vec_2_get::lua_vec_2_get;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_newindex(l: *mut lua_State) -> i32 {
  unsafe {
    let v = lua_vec_2_get(l, 1);
    let name_ptr = luaL_checkstring!(l, 2);
    let name = CStr::from_ptr(name_ptr as *const c_char)
      .to_str()
      .unwrap_or("");
    let value = luaL_checknumber(l, 3) as f32;

    if name == "X" {
      (*v).x = value;
    } else if name == "Y" {
      (*v).y = value;
    } else {
      lua_l_error_l(
        l,
        c"%s is not a writable member of vec2".as_ptr(),
        core::format_args!("{} is not a writable member of vec2", name),
      );
    }

    0
  }
}
