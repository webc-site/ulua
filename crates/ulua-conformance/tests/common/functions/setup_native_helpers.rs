use core::ffi::c_int;

use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use ulua_vm::{
  functions::{
    lua_g_isnative::luaG_isnative, lua_pushboolean::lua_pushboolean,
    lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::common::functions::run_conformance::CODEGEN;
unsafe extern "C-unwind" fn is_native(l: *mut lua_State) -> c_int {
  unsafe {
    lua_pushboolean(l, luaG_isnative(l, 1));
    1
  }
}

unsafe extern "C-unwind" fn is_native_if_supported(l: *mut lua_State) -> c_int {
  unsafe {
    if !CODEGEN || luau_codegen_supported() == 0 {
      lua_pushboolean(l, 1);
    } else {
      lua_pushboolean(l, luaG_isnative(l, 1));
    }

    1
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn setup_native_helpers(l: *mut lua_State) {
  unsafe {
    let is_native_fn: LuaCfunction =
      Some(is_native as unsafe extern "C-unwind" fn(*mut lua_State) -> c_int);
    lua_pushcclosurek(l, is_native_fn, c"is_native".as_ptr(), 0, None);
    lua_setglobal(l, c"is_native".as_ptr());

    let is_native_if_supported_fn: LuaCfunction =
      Some(is_native_if_supported as unsafe extern "C-unwind" fn(*mut lua_State) -> c_int);
    lua_pushcclosurek(
      l,
      is_native_if_supported_fn,
      c"is_native_if_supported".as_ptr(),
      0,
      None,
    );
    lua_setglobal(l, c"is_native_if_supported".as_ptr());
  }
}
