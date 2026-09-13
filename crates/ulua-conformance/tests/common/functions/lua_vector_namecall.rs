use core::ffi::{CStr, c_char};

use ulua_vm::{
  functions::{lua_l_error_l::lua_l_error_l, lua_namecallatom::lua_namecallatom},
  macros::lua_l_checkstring::luaL_checkstring,
  records::lua_state::lua_State,
};

use crate::common::functions::{
  lua_vector_cross::lua_vector_cross, lua_vector_dot::lua_vector_dot,
};

/// # Safety
///
/// `l` 必须是指向有效 `lua_State` 的指针。
pub unsafe extern "C-unwind" fn lua_vector_namecall(l: *mut lua_State) -> i32 {
  let mut atom: i32 = 0;
  let str_ptr = unsafe { lua_namecallatom(l, &mut atom) };

  if !str_ptr.is_null() {
    let str_cstr = unsafe { CStr::from_ptr(str_ptr) };
    let str_slice = str_cstr.to_str().unwrap_or("");

    if str_slice == "Dot" {
      return lua_vector_dot(l);
    }

    if str_slice == "Cross" {
      return lua_vector_cross(l);
    }
  }

  let arg1_ptr = unsafe { luaL_checkstring!(l, 1) };
  let arg1_cstr = unsafe { CStr::from_ptr(arg1_ptr as *const c_char) };
  let arg1_str = arg1_cstr.to_str().unwrap_or("");

  unsafe {
    lua_l_error_l(
      l,
      c"%s is not a valid method of vector".as_ptr(),
      core::format_args!("{} is not a valid method of vector", arg1_str),
    );
  }

  0
}
