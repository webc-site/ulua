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
  lua_vec_2_clone::lua_vec_2_clone, lua_vec_2_dot::lua_vec_2_dot, lua_vec_2_get::lua_vec_2_get,
  lua_vec_2_min::lua_vec_2_min, lua_vec_2_reenter::lua_vec_2_reenter,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_vec_2_namecall(l: *mut lua_State) -> i32 {
  unsafe {
    let str_ptr = lua_namecallatom(l, null_mut());

    if !str_ptr.is_null() {
      let str_cstr = CStr::from_ptr(str_ptr);
      let str_slice = str_cstr.to_str().unwrap_or("");

      if str_slice == "Dot" {
        let self_ptr = lua_vec_2_get(l, 1);
        return lua_vec_2_dot(l, self_ptr);
      }

      if str_slice == "Min" {
        let self_ptr = lua_vec_2_get(l, 1);
        return lua_vec_2_min(l, self_ptr);
      }

      if str_slice == "Clone" {
        let self_ptr = lua_vec_2_get(l, 1);
        return lua_vec_2_clone(l, self_ptr);
      }

      if str_slice == "Reenter" {
        let self_ptr = lua_vec_2_get(l, 1);
        return lua_vec_2_reenter(l, self_ptr);
      }
    }

    let arg1_ptr = luaL_checkstring!(l, 1);
    let arg1_cstr = CStr::from_ptr(arg1_ptr as *const c_char);
    let arg1_str = arg1_cstr.to_str().unwrap_or("");

    lua_l_error_l(
      l,
      c"%s is not a valid method of vector".as_ptr(),
      core::format_args!("{} is not a valid method of vector", arg1_str),
    );

    0
  }
}
