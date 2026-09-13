use core::{
  ffi::{CStr, c_char},
  ptr::null_mut,
};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_namecallatom::lua_namecallatom},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::functions::{
  lua_vertex_clone::lua_vertex_clone, lua_vertex_get::lua_vertex_get,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vertex_namecall(l: *mut lua_State) -> i32 {
  unsafe {
    let str_ptr = lua_namecallatom(l, null_mut());

    if !str_ptr.is_null() {
      let self_ptr = lua_vertex_get(l, 1);
      let str_cstr = CStr::from_ptr(str_ptr);
      let str_slice = str_cstr.to_str().unwrap_or("");

      if str_slice == "Clone" {
        return lua_vertex_clone(l, self_ptr);
      }
    }

    let arg1_ptr = luaL_checkstring!(l, 1);
    let arg1_cstr = CStr::from_ptr(arg1_ptr as *const c_char);
    let arg1_str = arg1_cstr.to_str().unwrap_or("");

    lua_l_error_l(
      l,
      c"%s is not a valid method of vertex".as_ptr(),
      core::format_args!("{} is not a valid method of vertex", arg1_str),
    );

    0
  }
}
